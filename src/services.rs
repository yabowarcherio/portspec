//! A small built-in table of well-known service-name → port mappings.
//!
//! Covers the protocols a network scanner would routinely run against —
//! `ssh`, `http`, `https`, and the like — without the weight of the full
//! IANA registry. Names are matched case-insensitively.

/// One row of the embedded service table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Service {
    /// The canonical lower-case service name.
    pub name: &'static str,
    /// The TCP/UDP port the service is assigned to.
    pub port: u16,
}

/// The embedded service table, sorted by `name`.
pub const SERVICES: &[Service] = &[
    Service { name: "amqp", port: 5672 },
    Service { name: "bgp", port: 179 },
    Service { name: "cassandra", port: 9042 },
    Service { name: "couchdb", port: 5984 },
    Service { name: "dhcp-client", port: 68 },
    Service { name: "dhcp-server", port: 67 },
    Service { name: "dns", port: 53 },
    Service { name: "elasticsearch", port: 9200 },
    Service { name: "etcd", port: 2379 },
    Service { name: "ftp", port: 21 },
    Service { name: "ftp-data", port: 20 },
    Service { name: "git", port: 9418 },
    Service { name: "grpc", port: 50051 },
    Service { name: "http", port: 80 },
    Service { name: "https", port: 443 },
    Service { name: "imap", port: 143 },
    Service { name: "imaps", port: 993 },
    Service { name: "ipp", port: 631 },
    Service { name: "irc", port: 6667 },
    Service { name: "kafka", port: 9092 },
    Service { name: "ldap", port: 389 },
    Service { name: "ldaps", port: 636 },
    Service { name: "memcached", port: 11211 },
    Service { name: "mongodb", port: 27017 },
    Service { name: "mqtt", port: 1883 },
    Service { name: "mssql", port: 1433 },
    Service { name: "mysql", port: 3306 },
    Service { name: "nats", port: 4222 },
    Service { name: "netbios-ns", port: 137 },
    Service { name: "netbios-ssn", port: 139 },
    Service { name: "ntp", port: 123 },
    Service { name: "pop3", port: 110 },
    Service { name: "pop3s", port: 995 },
    Service { name: "postgres", port: 5432 },
    Service { name: "prometheus", port: 9090 },
    Service { name: "rdp", port: 3389 },
    Service { name: "redis", port: 6379 },
    Service { name: "smb", port: 445 },
    Service { name: "smtp", port: 25 },
    Service { name: "smtps", port: 465 },
    Service { name: "snmp", port: 161 },
    Service { name: "ssh", port: 22 },
    Service { name: "submission", port: 587 },
    Service { name: "syslog", port: 514 },
    Service { name: "telnet", port: 23 },
    Service { name: "tftp", port: 69 },
    Service { name: "vnc", port: 5900 },
    Service { name: "winrm", port: 5985 },
    Service { name: "winrm-https", port: 5986 },
    Service { name: "zookeeper", port: 2181 },
];

/// Look up the port for a service by name. Match is case-insensitive.
pub fn port_for(name: &str) -> Option<u16> {
    SERVICES
        .iter()
        .find(|s| s.name.eq_ignore_ascii_case(name))
        .map(|s| s.port)
}

/// Look up the service name for a port, or `None` if the port isn't in the
/// built-in table.
///
/// When several services share a port, returns the first match in
/// alphabetical order (the table's sort order).
pub fn service_for(port: u16) -> Option<&'static str> {
    SERVICES.iter().find(|s| s.port == port).map(|s| s.name)
}

/// Every name registered to `port` in the built-in table. Returns an empty
/// iterator when the port has no matching service.
///
/// Several entries can share a port (e.g. `submission` and `smtps` could
/// both live on different ports but a single port could host multiple
/// labelled services). This iterator yields every match.
pub fn services_for(port: u16) -> impl Iterator<Item = &'static str> {
    SERVICES
        .iter()
        .filter(move |s| s.port == port)
        .map(|s| s.name)
}

/// The number of services in the embedded table.
pub const SERVICES_COUNT: usize = SERVICES.len();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn well_known_lookups() {
        assert_eq!(port_for("ssh"), Some(22));
        assert_eq!(port_for("SSH"), Some(22));
        assert_eq!(port_for("https"), Some(443));
        assert_eq!(port_for("nope"), None);
    }

    #[test]
    fn reverse_lookups() {
        assert_eq!(service_for(22), Some("ssh"));
        assert_eq!(service_for(443), Some("https"));
        assert_eq!(service_for(0), None);
    }

    #[test]
    fn table_is_sorted_by_name() {
        for w in SERVICES.windows(2) {
            assert!(w[0].name < w[1].name, "{:?} > {:?}", w[0], w[1]);
        }
    }
}
