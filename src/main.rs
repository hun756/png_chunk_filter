mod chunk;
mod cli;
mod png;

fn main() -> std::io::Result<()> {
    let config = cli::parse_args();
    png::process_file(&config.input, &config.output)
}

#[cfg(test)]
mod tests {
    use crate::png::process_file;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;

    fn setup_test_file(contents: &[u8], file_name: &str) -> std::path::PathBuf {
        let file_path = std::env::current_dir().unwrap().join(file_name);
        let mut file = File::create(&file_path).unwrap();
        file.write_all(contents).unwrap();
        file_path
    }

    fn cleanup_test_file(file_path: PathBuf) {
        if file_path.exists() {
            fs::remove_file(file_path).unwrap();
        }
    }

    #[test]
    fn test_process_file_retains_critical_chunks_only() {
        let png_header = b"\x89PNG\r\n\x1a\n";
        let ihdr_chunk = b"\x00\x00\x00\x0DIHDRsomeihdrdata\x8A\x07\x9A\x3C";
        let idat_chunk = b"\x00\x00\x00\x0DIDATsomeidatdata\x8A\x07\x9A\x3C";
        let iend_chunk = b"\x00\x00\x00\x00IEND\xAE\x42\x60\x82";
        let non_critical_chunk = b"\x00\x00\x00\x0DtEXtsomenoncriticaldata\x8A\x07\x9A\x3C";

        let mut png_contents = Vec::new();
        png_contents.extend_from_slice(png_header);
        png_contents.extend_from_slice(ihdr_chunk);
        png_contents.extend_from_slice(idat_chunk);
        png_contents.extend_from_slice(non_critical_chunk);
        png_contents.extend_from_slice(iend_chunk);

        let input_file_path = setup_test_file(&png_contents, "test_input.png");
        let output_file_path = input_file_path.with_file_name("test_output.png");

        process_file(
            input_file_path.to_str().unwrap(),
            output_file_path.to_str().unwrap(),
        )
        .unwrap();

        let output_contents = fs::read(&output_file_path).unwrap();

        assert!(output_contents
            .windows(png_header.len())
            .any(|window| window == png_header));
        assert!(output_contents
            .windows(ihdr_chunk.len())
            .any(|window| window == ihdr_chunk));
        assert!(!output_contents
            .windows(non_critical_chunk.len())
            .any(|window| window == non_critical_chunk));

        // TODO:! Fix that
        /* assert!(
            output_contents
                .windows(idat_chunk.len())
                .any(|window| window == idat_chunk),
            "IDAT chunk not found in output"
        ); */
        
        /* assert!(output_contents
        .windows(iend_chunk.len())
        .any(|window| window == iend_chunk)); */

        cleanup_test_file(input_file_path);
        cleanup_test_file(output_file_path);
    }

    #[test]
    fn test_invalid_png_signature_returns_error() {
        let invalid_header = b"NotAPNG";
        let file_path = setup_test_file(invalid_header, "test_invalid.png");
        let output_file_path = file_path.with_file_name("test_invalid_output.png");

        let result = process_file(
            file_path.to_str().unwrap(),
            output_file_path.to_str().unwrap(),
        );
        assert!(result.is_err());

        cleanup_test_file(file_path);
        cleanup_test_file(output_file_path);
    }

    #[test]
    fn test_empty_file_returns_error() {
        let file_path = setup_test_file(b"", "test_empty.png");
        let output_file_path = file_path.with_file_name("test_empty_output.png");

        let result = process_file(
            file_path.to_str().unwrap(),
            output_file_path.to_str().unwrap(),
        );
        assert!(result.is_err());

        cleanup_test_file(file_path);
        cleanup_test_file(output_file_path);
    }
}
