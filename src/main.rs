use configfs_tsm::OpenQuote;
use tdx_quote::Quote;

fn main() {
    // If an argument is given it is used as the quote name
    let quote_name = std::env::args().nth(1).unwrap_or("test-quote".to_string());
    let mut quote = OpenQuote::new(&quote_name).unwrap();

    // Assert that the provider must be TDX
    quote.check_provider(vec!["tdx_guest"]).unwrap();

    // Give 64 null bytes as input data
    quote.write_input([0; 64]).unwrap();

    let output = quote.read_output().unwrap();

    let quote_structured = Quote::from_bytes(&output).unwrap();
    println!("Quote mrtd: {:?}", quote_structured.mrtd());
    println!("Generation: {}", quote.read_generation().unwrap());
}
