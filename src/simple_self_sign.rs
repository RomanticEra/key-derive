#[cfg(test)]
mod test {
    use rcgen::RcgenError;

    use std::fs::write;
    #[test]
    fn generate_self_signed_cert() -> Result<(), RcgenError> {
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;

        write("s_ca.pem", cert.serialize_pem()?).unwrap();
        write("s_key.pem", cert.serialize_private_key_pem()).unwrap();
        Ok(())
    }
}
