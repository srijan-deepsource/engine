use trust_dns_resolver::proto::rr::{RecordType, RData};

#[test]
fn test_bad_add() {
    use std::net::*;
    use trust_dns_resolver::Resolver;
    use trust_dns_resolver::config::*;

// Construct a new Resolver with default configuration options
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let ret = resolver.lookup("www.fillbit.io.", RecordType::CNAME).unwrap();

    for record in ret.record_iter() {
        match record.rdata() {
            RData::CNAME(name) => eprintln!("{:?}", name.to_utf8()),
            _ => {}
        }

    }
}