use std::{fs::File, io::Read};

use criterion::{criterion_group, criterion_main, Criterion};
use image::{codecs::jpeg::JpegDecoder, DynamicImage};

macro_rules! gen_benches {
    ($c:expr, $($name:literal -> $data:expr),*) => {
      {
        let mut image_rs_bench = $c.benchmark_group("image-rs");

        $(
          image_rs_bench.bench_function($name, |f| {
              f.iter(|| {
                let decoded = JpegDecoder::new($data.as_ref()).unwrap();
                let _data = DynamicImage::from_decoder(decoded).unwrap();
              })
            });
        )*
      }

      {
        let mut turbojpeg_bench = $c.benchmark_group("turbojpeg");

        $(
          turbojpeg_bench.bench_function($name, |f| {
            f.iter(|| {
              let _decompressed: image::RgbImage = turbojpeg::decompress_image($data.as_ref()).unwrap();
            })
          });
        )*
      }

      {
        let mut mozjpeg_bench = $c.benchmark_group("mozpeg");

        $(
          mozjpeg_bench.bench_function($name, |f| {
            f.iter(|| {
              let image = mozjpeg::Decompress::new_mem($data.as_ref()).unwrap();
              let mut rgb = image.rgb().unwrap();

              let _pixels = rgb.read_scanlines::<[u8; 3]>().unwrap();
              assert!(rgb.finish_decompress());
            })
          });
        )*
      }

      /* TODO: Zune complains about "Marker missing when expected"
      {
        let mut zune_bench = $c.benchmark_group("zune-jpeg");

        $(
          zune_bench.bench_function($name, |f| {
            f.iter(|| {
              let _pixels = zune_jpeg::Decoder::new().decode_buffer($data.as_ref()).unwrap();
            })
          });
        )*
      }
      */
    };
}

macro_rules! read_image {
    ($path:literal) => {{
        let mut data_vec = vec![];
        File::open($path)
            .unwrap()
            .read_to_end(&mut data_vec)
            .unwrap();
        data_vec.into_boxed_slice()
    }};
}

fn bench_image(c: &mut Criterion) {
    let image_1_data = read_image!("images/1.jpg");
    let image_2_data = read_image!("images/2.jpg");

    gen_benches!(c,
      "Image 1" -> image_1_data,
      "Image 2" -> image_2_data
    );
}

criterion_group!(benches, bench_image);
criterion_main!(benches);
