extern crate bencoderus;
extern crate hyper;

use std::io::Read;
use std::net::Ipv4Addr;

use hyper::{Client, Url};

use bencoderus::Bencoding;

fn main() {

    let mut url = "".to_owned();

    let host = 
        //"http://mgtracker.org:6969";
        //"http://5.79.83.193:2710";
        //"http://182.176.139.129:6969"
        //"http://5.79.83.193:2710"
        //"http://91.218.230.81:6969"
        //"http://atrack.pow7.com"
        //"http://open.touki.ru"
        //"http://p4p.arenabg.ch:1337"

        //"http://retracker.krs-ix.ru:80"
        //"http://thetracker.org:80"
        //"http://torrentsmd.com:8080"
        //"http://tracker.bittor.pw:1337"
        //"http://tracker.dutchtracking.com:80"
        //"http://tracker.edoardocolombo.eu:6969"
        //"http://tracker.kicks-ass.net:80"
        //"http://1337x.org"
        //"http://leechers-paradise.org"
        //"http://tracker.kicks-ass.net"
        "http://tracker.opentrackr.org:1337"
        ;

    let query = "/announce?info_hash=%3E%09%D0%1CHHCy%CF%27%F23%24%E1%7B%BDijbD&peer_id=76433642664923430920&port=56723&uploaded=0&downloaded=0&left=0&event=started&compact=1";

    url.push_str(host);
    url.push_str(query);

    println!("URL: {}", url);

    // Note:
    // peers: with the compact option specified in the querystring, this is a byte[].
    // Every 6 bytes is a peer
    // – the first four are the four numbers in an IPv4 address
    // - and the last two are a big endian char representing the port number.
    //

    let url = match Url::parse(&url) {
        Ok(url) => url,
        Err(_) => panic!("Unable to parse url."),
    };

    if url.scheme() != "http" {
        println!("HTTPS not yet implemented.");
        return;
    }

    let client = Client::new();

    let mut res = client.get(url).send().unwrap();
    assert_eq!(res.status, hyper::Ok);
    println!("Response Status: {:?}", res.status);

    println!("Response: {:?}", res);

    let mut s: Vec<u8> = vec![];
    let sz = res.read_to_end(&mut s);
    println!("response size: {:?}", sz);
    println!("Resp body: {:?}", s);

    println!("Decode bencoding:");

    let b = bencoderus::libdecode::decode(s);
    println!("DECODED:");
    println!("{:?}", b);
    let d = match b {
        Bencoding::Dictionary(v) => Some(v),
        _ => None,
    };
    let du = d.unwrap();
    println!("du :: {:?}", du);
    let key_vec = "peers".as_bytes();
    let peers = &du[key_vec];

    let ps = match peers {
        &Bencoding::ByteString(ref v) => v,
        _ => {
            "oopsie".as_bytes()
        },
    };
    
    #[derive(Debug)]
    struct PeerAddr {
        ip_addr: Ipv4Addr,
        port: u16,
    }

    let mut peers: Vec<PeerAddr> = vec![];
    for ch in ps.chunks(6) {
        let ip_val = Ipv4Addr::new(ch[0], ch[1], ch[2], ch[3]);
        let port_val = (ch[4] as u16) << 8 | (ch[5] as u16);
        peers.push(PeerAddr{ip_addr: ip_val, port: port_val});
        println!("ip: {}.{}.{}.{} : {} {}", ch[0], ch[1], ch[2], ch[3], ch[4], ch[5]);
    }
    println!("peers:");
    for p in peers{
        println!("{:?}", p);
    }
    println!("--------");
}
