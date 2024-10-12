use crate::args::Args;
use ab_glyph::{FontRef, PxScale};
use anyhow::{bail, Context, Result};
use image::{EncodableLayout, ImageBuffer, Rgb};
use imageproc::drawing;
use qrcode::QrCode;
use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

pub struct App {}

impl App {
    pub fn run(args: Args) -> Result<()> {
        // Step 1: Encrypt the input
        let encrypted = App::age(&args)?;

        // Step 2 (optional): Write the encrypted blob to file/stdout
        match &args.age_output {
            None => {}
            Some(path) if path == Path::new("-") => print!("{}", encrypted),
            Some(path) => std::fs::write(path, &encrypted)?,
        }

        // Step 3: Generate QR code
        let image = App::qr_code(&encrypted, &args)?;

        // Step 4: Write the result to file/stdout
        match args.output {
            path if path == Path::new("-") => std::io::stdout().write_all(image.as_bytes())?,
            path => image.save(path)?,
        }

        Ok(())
    }

    fn age(args: &Args) -> Result<String> {
        let mut cmd = Command::new(args.age_bin.clone());

        args.recipient
            .iter()
            .fold(&mut cmd, |cmd, arg| cmd.arg("-r").arg(arg));
        args.recipient_file
            .iter()
            .fold(&mut cmd, |cmd, file| cmd.arg("-R").arg(file));

        let mut proc = cmd
            .arg("-e")
            .arg("-a")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let content = args
            .input
            .clone()
            .contents()
            .with_context(|| format!("Could not read seal input: {}", args.input.filename()))?;

        proc.stdin
            .take()
            .context("Could not open rage's stdin")?
            .write_all(content.as_bytes())?;

        let output = proc.wait_with_output()?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            if stdout.is_empty() {
                bail!("rage returned empty result")
            } else {
                Ok(stdout)
            }
        } else {
            bail!(
                "rage exited with exit code: {}\nstdout: {}\nstderr: {}",
                output.status,
                String::from_utf8_lossy(&output.stdout).to_string(),
                String::from_utf8_lossy(&output.stderr).to_string()
            )
        }
    }

    fn qr_code(blob: &str, args: &Args) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        let qr = QrCode::new(blob.as_bytes())
            .context("Could not generate QR code from the encrypted blob")?;
        let mut image = qr.render::<Rgb<u8>>().module_dimensions(4, 4).build();

        if let Some(label) = &args.label {
            let font = FontRef::try_from_slice(include_bytes!("../resources/DejaVuSans.ttf"))
                .context("Could not load the font")?;
            let scale = PxScale {
                x: args.font_size,
                y: args.font_size,
            };
            let (org_width, org_height) = image.dimensions();
            let (text_width, text_height) = drawing::text_size(scale, &font, label);

            let new_height = org_height + (3 * text_height / 2);

            let mut extended_img =
                ImageBuffer::from_pixel(org_width, new_height, Rgb([255, 255, 255]));

            image::imageops::overlay(&mut extended_img, &image, 0, 0);
            image = extended_img;

            drawing::draw_text_mut(
                &mut image,
                Rgb([0u8, 0u8, 0u8]),
                (org_width / 2 - text_width / 2) as i32,
                org_height as i32,
                scale,
                &font,
                label,
            );
        }

        Ok(image)
    }
}
