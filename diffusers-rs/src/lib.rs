use std::path::Path;

use diffusers::pipelines::stable_diffusion;
use diffusers::transformers::clip;
use tch::{nn::Module, Device, Kind, Tensor};

const GUIDANCE_SCALE: f64 = 7.5;
const SEED: i64 = 32;

pub fn generate_image(
    prompt: &str,
    output: &str,
    n_steps: usize,
    data_dir: &Path,
) -> anyhow::Result<()> {
    tch::maybe_init_cuda();
    println!(
        "Using device: {}",
        if Device::cuda_if_available() == Device::Cpu {
            "CPU"
        } else {
            "CUDA"
        }
    );

    let sd_config = stable_diffusion::StableDiffusionConfig::v2_1(None, None, None);

    let device_setup = diffusers::utils::DeviceSetup::new(vec![]);
    let clip_device = device_setup.get("clip");
    let vae_device = device_setup.get("vae");
    let unet_device = device_setup.get("unet");

    let vocab_path = data_dir.join("bpe_simple_vocab_16e6.txt");
    let clip_weights = data_dir.join("clip_v2.1.safetensors");
    let vae_weights = data_dir.join("vae_v2.1.safetensors");
    let unet_weights = data_dir.join("unet_v2.1.safetensors");

    println!("Building scheduler...");
    let scheduler = sd_config.build_scheduler(n_steps);

    println!("Loading tokenizer...");
    let tokenizer =
        clip::Tokenizer::create(vocab_path.to_str().unwrap(), &sd_config.clip)?;

    let tokens = tokenizer.encode(prompt)?;
    let tokens: Vec<i64> = tokens.into_iter().map(|x| x as i64).collect();
    let tokens = Tensor::from_slice(&tokens).view((1, -1)).to(clip_device);

    let uncond_tokens = tokenizer.encode("")?;
    let uncond_tokens: Vec<i64> = uncond_tokens.into_iter().map(|x| x as i64).collect();
    let uncond_tokens = Tensor::from_slice(&uncond_tokens)
        .view((1, -1))
        .to(clip_device);

    let _no_grad_guard = tch::no_grad_guard();

    println!("Loading CLIP text model...");
    let text_model = sd_config
        .build_clip_transformer(clip_weights.to_str().unwrap(), clip_device)?;

    println!("Loading VAE...");
    let vae = sd_config.build_vae(vae_weights.to_str().unwrap(), vae_device)?;

    println!("Loading UNet...");
    let unet =
        sd_config.build_unet(unet_weights.to_str().unwrap(), unet_device, 4)?;

    println!("Computing text embeddings...");
    let text_embeddings = text_model.forward(&tokens);
    let uncond_embeddings = text_model.forward(&uncond_tokens);
    let text_embeddings =
        Tensor::cat(&[uncond_embeddings, text_embeddings], 0).to(unet_device);

    tch::manual_seed(SEED);
    let mut latents = Tensor::randn(
        [1, 4, sd_config.height / 8, sd_config.width / 8],
        (Kind::Float, unet_device),
    );
    latents *= scheduler.init_noise_sigma();

    println!("Running {n_steps} denoising steps...");
    for (idx, &timestep) in scheduler.timesteps().iter().enumerate() {
        println!("  Step {}/{n_steps}", idx + 1);

        let latent_model_input = Tensor::cat(&[&latents, &latents], 0);
        let latent_model_input = scheduler.scale_model_input(latent_model_input, timestep);

        let noise_pred =
            unet.forward(&latent_model_input, timestep as f64, &text_embeddings);

        let noise_pred = noise_pred.chunk(2, 0);
        let (noise_pred_uncond, noise_pred_text) = (&noise_pred[0], &noise_pred[1]);
        let noise_pred =
            noise_pred_uncond + (noise_pred_text - noise_pred_uncond) * GUIDANCE_SCALE;

        latents = scheduler.step(&noise_pred, timestep, &latents);
    }

    println!("Decoding latents to image...");
    let latents = latents.to(vae_device);
    let image = vae.decode(&(&latents / 0.18215));
    let image = (image / 2 + 0.5).clamp(0., 1.).to_device(Device::Cpu);
    let image = (image * 255.).to_kind(Kind::Uint8);

    tch::vision::image::save(&image, output)?;
    println!("Image saved to {output}");

    Ok(())
}
