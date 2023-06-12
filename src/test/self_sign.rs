//! source from https://github.com/djc/sign-cert-remote
#[cfg(test)]
mod test {
    use std::fs::write;

    use rcgen::{
        BasicConstraints, Certificate, CertificateParams, CertificateSigningRequest, DnType, IsCa,
    };
    use x509_parser::certification_request::X509CertificationRequest;
    use x509_parser::prelude::FromDer;
    #[test]
    fn main() {
        let ca = Ca::new();
        println!("writing CA certificate to ca.pem...");
        write("ca.pem", ca.certificate.serialize_pem().unwrap()).unwrap();
        write("key.pem", ca.certificate.serialize_private_key_pem()).unwrap();

        // println!("{}", cert.serialize_private_key_pem());

        let entity = Entity::new();
        println!("writing CSR for entity to csr.pem...");
        let csr = entity.create_csr();
        write("csr.pem", &csr).unwrap();

        println!("writing directly signed certificate to direct.pem...");
        let direct = entity
            .certificate
            .serialize_pem_with_signer(&ca.certificate)
            .unwrap();
        write("direct.pem", direct).unwrap();

        println!("writing certificate created from CSR to indirect.pem...");
        let indirect = ca.create_cert(&csr);
        write("indirect.pem", indirect).unwrap();
    }

    struct Ca {
        certificate: Certificate,
    }

    impl Ca {
        fn new() -> Self {
            let mut params = CertificateParams::new(vec!["huang12zheng.github.io".to_owned()]);
            let dn = &mut params.distinguished_name;
            dn.push(DnType::CommonName, "ca.huang12zheng.github.io");

            params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
            Self {
                certificate: Certificate::from_params(params).unwrap(),
            }
        }

        fn create_cert(&self, csr_pem: &str) -> String {
            let csr_der = x509_parser::pem::parse_x509_pem(csr_pem.as_bytes())
                .unwrap()
                .1;
            let csr = X509CertificationRequest::from_der(&csr_der.contents)
                .unwrap()
                .1;
            csr.verify_signature().unwrap();
            let csr = CertificateSigningRequest::from_der(&csr_der.contents).unwrap();
            csr.serialize_pem_with_signer(&self.certificate).unwrap()
        }
    }

    struct Entity {
        certificate: Certificate,
    }

    impl Entity {
        fn new() -> Self {
            let params = CertificateParams::new(vec![
                "huang12zheng.github.io".to_owned(),
                "localhost".to_owned(),
            ]);

            // let dn = &mut params.distinguished_name;
            // dn.push(DnType::CommonName, "huang12zheng.github.io");
            // dn.push(DnType::CommonName, "localhost");
            Self {
                certificate: Certificate::from_params(params).unwrap(),
            }
        }

        fn create_csr(&self) -> String {
            self.certificate.serialize_request_pem().unwrap()
        }
    }
}