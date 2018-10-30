extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate rustc_serialize;
extern crate uuid;
extern crate url;

use hyper::{Response, Request, Method, Server, Client, StatusCode, Body, header, Chunk};
use hyper::service::service_fn;
use hyper::client::HttpConnector;
use futures::{future, Future};
use serde::ser;
use futures::Stream;
use std::io::Write;


extern crate time;
extern crate crypto;



#[macro_use] 
extern crate lazy_static;

use std::sync::Mutex;


use rustc_serialize::hex::ToHex;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use uuid::Uuid;
use std::str;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct NewMessage {
    pub username: String,
    pub message: String,
}

#[derive(PartialEq, PartialOrd)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Transaction {
    amount: u32,
    recipient: String,
    sender: String
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Block {
    index: u32,
    previous_hash: String,
    proof: u32,
    timestamp: u32,
    transactions: Vec<Transaction>
}

pub struct Blockchain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
    node_identifier: String
}


impl Blockchain {
    fn new() -> Blockchain {
        let mut vec: Vec<Block> = Vec::new();
        vec.push(Block {
            index: 1,
            previous_hash: "1".to_string(),
            proof: 100,
            timestamp: 333037375,
            transactions: Vec::new()
        });
        let my_uuid = Uuid::new_v4();
        Blockchain { chain: vec, current_transactions: Vec::new(), node_identifier: my_uuid.to_string().replace("-", "") }
    }
    pub fn new_transaction(&mut self, transaction: Transaction) -> u32 {
        self.current_transactions.push(transaction);
        let num = match self.chain.last()
            {
                Some(_) => self.chain.last().unwrap().index,
                None => 0
            };
        num + 1
    }

    pub fn new_block(&mut self, proof: u32, previous_hash: String) -> Block {
        let block = Block {
            index: self.chain.last().unwrap().index + 1,
            previous_hash: previous_hash,
            proof: proof,
            timestamp: time::now().tm_nsec as u32,
            transactions: self.current_transactions.to_vec()
        };

        self.chain.push(block.clone());
        self.current_transactions = Vec::new();
        block
    }

    pub fn calculate_hash(input: &String) -> String {
        let mut sha = Sha256::new();
        sha.input_str(&input);
        sha.result_str().as_bytes().to_hex()
    }

    pub fn calculate_hash_from_block(input: &Block) -> String {
        let json = serde_json::to_string(input).expect("Couldn't serialize block");
        Blockchain::calculate_hash(&json)
    }

    fn valid_proof(proof: u32, last_proof: u32) -> bool {
        let guess = format!("{}{}", proof, last_proof);
        let mut sha = Sha256::new();
        sha.input_str(&guess);
        let guess_hash = sha.result_str();
        guess_hash[0..4].eq("0000")
    }

    pub fn proof_of_work(last_proof: u32) -> u32 {
        let mut proof = 0;
        while Blockchain::valid_proof(proof, last_proof) == false {
            proof += 1;
        }
        proof
    }

    fn reward_for_proof(&mut self) {
        self.current_transactions.push(Transaction { amount: 1, recipient: self.node_identifier.clone(), sender: "0".to_string() });
    }

    pub fn mine_new_block(&mut self) -> Block {
        let last_block = self.chain.last().unwrap().clone();
        let proof = Blockchain::proof_of_work(last_block.proof);
        let previous_hash = Blockchain::calculate_hash_from_block(&last_block);
        self.reward_for_proof();
        self.new_block(proof, previous_hash)
    }
}

fn get_transaction(data: &str) -> Result<Transaction, serde_json::Error> {
    let p: Transaction = serde_json::from_str(data)?;
    println!("Transaction to add: {}, {}, {}", p.amount, p.recipient, p.sender);
    Ok(p)
}

fn parse_form(form_chunk: Result<Chunk, hyper::Error>) -> futures::future::FutureResult<hyper::Response<Body>, hyper::Error>  {
    match form_chunk {
        Ok(body) => {
            let mut acc = Vec::new();
            acc.extend_from_slice(&*body);
            let stringify = String::from_utf8(acc).unwrap();

            match get_transaction(&stringify.to_string()) {
                Ok(transaction) => {
                    let mut guard = GLOBAL_BLOCKCHAIN.lock().unwrap();
                    let block = guard.new_transaction(transaction);
                    let payload = json!({"message" : format!("Transaction will be added to block {}", block)}).to_string();
                    let response = Response::builder()
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(payload))
                        .unwrap();
                    debug!("{:?}", response);
                    futures::future::ok(response)
                }
                Err(_) => {
                    future::ok(make_error_response("Received JSON error"))
//                    futures::future::err(hyper::Error::from(std::io::Error::new(
//                        std::io::ErrorKind::InvalidInput,
//                        "Missing field 'amount' or 'recipient' or 'sender'",
//                    )))
                }
            }
        }
        Err(error) => {
            future::ok(make_error_response("unknown error"))
        }
    }
}


fn make_error_response(error_message: &str) -> Response<Body> {
    let payload = json!({
        "error": error_message
    }).to_string();
    Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload))
        .unwrap()
}

fn return_json<T>(data: &T) -> hyper::Response<Body>
    where T: ser::Serialize
{
    match serde_json::to_string(data) {
        Ok(json) => {
            Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json))
                .unwrap()
        }
        Err(e) => {
            eprintln!("serializing json: {}", e);

            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal Server Error"))
                .unwrap()
        }
    }
}

#[derive(Serialize, Deserialize)]
struct NodeList {
    nodes: Vec<String>,
}

fn typed_example(data: &str) -> Result<NodeList, serde_json::Error> {
    let p: NodeList = serde_json::from_str(data)?;
    println!("Please register: {:?}", p.nodes);
    Ok(p)
}


fn parse_register(form_chunk: Result<Chunk, hyper::Error>) -> futures::future::FutureResult<hyper::Response<Body>, hyper::Error> {
    match form_chunk {
        Ok(body) => {
            let mut acc = Vec::new();
            acc.extend_from_slice(&*body);
            let stringify = String::from_utf8(acc).unwrap();
            println!("{}", stringify);

            match typed_example(&stringify.to_string()) {
                Ok(json) => {
                    for item in json.nodes {
                        register_node(item);
                    }

                    let all = GLOBAL_NODES_SET.lock().unwrap().to_vec();
                    let nodes_str = format!("{:?}", all);
                    let payload = json!({"message" : format!("Nodes will be registered"), "nodes" : nodes_str}).to_string();
                    debug!("{:?}", payload);
                    future::ok(Response::builder()
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(payload))
                        .unwrap())
                }
                Err(_) => {
                    future::ok(make_error_response("Wrong nodes list"))
                }
            }
            
        }
        Err(_) => {
            future::ok(make_error_response("No nodes in request"))
        }
    }
}

lazy_static! {
    static ref GLOBAL_BLOCKCHAIN: Mutex<Blockchain> = Mutex::new(Blockchain::new());
    static ref GLOBAL_NODES_SET: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

mod lib;

fn register_node(address: String) {
//    println!("{}", address);
    let result = GLOBAL_NODES_SET.lock().unwrap().iter().position(|r| r.to_string() == address);
    match  result {
        Some(_) => {}
        _ => {
            GLOBAL_NODES_SET.lock().unwrap().push(address);
        }
    }
}

fn request_chain_from(neighbour: String) -> Vec<Block> {
    println!("{}", neighbour);
    let url = neighbour.parse::<hyper::Uri>().unwrap();
    hyper::rt::run(fetch_url(url));

    Vec::new()
}

fn fetch_url(url: hyper::Uri) -> impl Future<Item=(), Error=()> {
    let client = Client::new();

    client
        // Fetch the url...
        .get(url)
        // And then, if we get a response back...
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: {:#?}", res.headers());

            // The body is a stream, and for_each returns a new Future
            // when the stream is finished, and calls the closure on
            // each chunk of the body...
            res.into_body().for_each(|chunk| {
                std::io::stdout().write_all(&chunk)
                    .map_err(|e| panic!("example expects stdout is open, error={}", e))
            })
        })
        // If all good, just tell the user...
        .map(|_| {
            println!("\n\nDone.");
        })
        // If there was an error, let the user know...
        .map_err(|err| {
            eprintln!("Error {}", err);
        })
}

fn chain_consensus() -> Vec<Block> {
    let neighbours = GLOBAL_NODES_SET.lock().unwrap();
    let mut tmp_longest_chain: Vec<Block> = GLOBAL_BLOCKCHAIN.lock().unwrap().chain.clone();

    for neighbour in neighbours.iter() {
        let new_chain = request_chain_from(neighbour.to_string());
        if new_chain.len() > GLOBAL_NODES_SET.lock().unwrap().len() {
            tmp_longest_chain = new_chain;
        }
    }

    tmp_longest_chain
}

fn make_resolve_response() -> futures::future::FutureResult<hyper::Response<Body>, hyper::Error> {
    let validated_chain = chain_consensus();
    let json_resp = json!({"blockchain" : json!(&validated_chain).to_string()}).to_string();

    let response = Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json_resp))
        .unwrap();

    future::ok(response)
}

fn response(req: Request<Body>, client: &Client<HttpConnector>)
    -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>{
    debug!("{:?}", req);
    static NOTFOUND: &[u8] = b"Not Found";

    match (req.method(), req.uri().path()){
        (&Method::GET, "/mine") => {
            let mut guard = GLOBAL_BLOCKCHAIN.lock().unwrap();
            let block = guard.mine_new_block();
            let res = return_json(&block);

            Box::new(future::ok(res))
        }
        (&Method::POST, "/transactions/new") => {
            let future = req
                .into_body()
                .concat2()
                .then(parse_form);
            Box::new(future)
        }
        (&Method::GET, "/chain") => {
            let chain = &GLOBAL_BLOCKCHAIN.lock().unwrap().chain;
            Box::new(future::ok(return_json(&chain)))
        }
        (&Method::POST, "/nodes/register") => {
            let body = req.into_body()
                .concat2()
                .then(parse_register);
            Box::new(body)
        }
        (&Method::GET, "/nodes/resolve") => {
            Box::new(make_resolve_response())
        }
        _ => {
            let body = Body::from(NOTFOUND);
            Box::new(future::ok(Response::builder()
                                         .status(StatusCode::NOT_FOUND)
                                         .body(body)
                                         .unwrap()))
        }
    }

}


fn main() {
    pretty_env_logger::init();

    let addr = "127.0.0.1:1337".parse().unwrap();

    hyper::rt::run(future::lazy(move || {
        // Share a `Client` with all `Service`s
        let client = Client::new();

        let new_service = move || {
            // Move a clone of `client` into the `service_fn`.
            let client = client.clone();
            service_fn(move |req| {
                response(req, &client)
            })
        };

        let server = Server::bind(&addr)
            .serve(new_service)
            .map_err(|e| eprintln!("server error: {}", e));

        println!("Listening on http://{}", addr);

        server
    }));
}





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use super::*;

    #[test]
    fn new_transaction_is_added() {
        let mut chain = Blockchain::new();

        let transaction = Transaction { amount: 5, recipient: "me".to_string(), sender: "you".to_string() };
        let index = chain.new_transaction(transaction);
        assert_eq!(1, chain.current_transactions.len());
        assert_eq!(index, 2);
    }

    #[test]
    fn last_index_is_not_incremented() {
        let mut chain = Blockchain::new();

        let transaction = Transaction { amount: 5, recipient: "me".to_string(), sender: "you".to_string() };
        let index = chain.new_transaction(transaction);
        assert_eq!(1, chain.current_transactions.len());
        let index2 = chain.new_transaction(Transaction { amount: 10, recipient: "you".to_string(), sender: "me".to_string() });
        assert_eq!(index, 2);
        assert_eq!(2, chain.current_transactions.len());
        assert_eq!(index2, 2);
    }

    #[test]
    pub fn test_to_hex() {
        assert_eq!("foobar".as_bytes().to_hex(), "666f6f626172");
    }

    #[test]
    pub fn test_calculate_hash()
    {
        let block = Block {
            index: 1,
            previous_hash: "1".to_string(),
            proof: 100,
            timestamp: 333037375,
            transactions: Vec::new()
        };
        let hash = Blockchain::calculate_hash_from_block(&block);
        let expected_hash = "36303566323332313133666639643032316337376264613461303932333666313\
            564366136366363363062613137636335393231643564393336636139653133";
        assert_eq!(hash, expected_hash);
    }

    #[test]
    pub fn test_proof_of_work()
    {
        assert_eq!(93711, Blockchain::proof_of_work(1));
    }

    #[test]
    fn last_index_is_incremented_when_block_is_mined() {
        let mut chain = Blockchain::new();
        let transaction = Transaction { amount: 5, recipient: "me".to_string(), sender: "you".to_string() };
        let index = chain.new_transaction(transaction);
        assert_eq!(1, chain.current_transactions.len());
        chain.mine_new_block();
        let index2 = chain.new_transaction(Transaction { amount: 10, recipient: "you".to_string(), sender: "me".to_string() });
        assert_eq!(1, chain.current_transactions.len());
        assert_eq!(index, 2);
        assert_eq!(index2, 3);
    }

    // #[test]
    // fn () {
    //     let mut chain = Blockchain::new();
    //     let transaction = Transaction { amount: 5, recipient: "me".to_string(), sender: "you".to_string() };
    //     let index = chain.new_transaction(transaction);
    //     assert_eq!(1, chain.current_transactions.len());
    //     chain.mine_new_block();
    //     let index2 = chain.new_transaction(Transaction { amount: 10, recipient: "you".to_string(), sender: "me".to_string() });
    //     assert_eq!(1, chain.current_transactions.len());


    // }
}
