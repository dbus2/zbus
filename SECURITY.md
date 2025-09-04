# Security Policy

## Supported Versions

We take security seriously and provide security updates for the latest version of zbus and
its ecosystem crates. We strongly recommend keeping your zbus dependencies up to date.

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

If you discover a security vulnerability in zbus or any of the related crates (zvariant,
zbus_names, zbus_macros, zbus_xml, zbus_xmlgen, etc.), please report it privately by emailing
**zeenixATgmail**.

Please include the following information in your report:

- A clear description of the vulnerability
- Steps to reproduce the issue
- Potential impact and attack scenarios
- Any suggested fixes or mitigations
- Your contact information for follow-up questions

### What constitutes a security vulnerability?

For zbus, security vulnerabilities may include but are not limited to:

- **Authentication bypass**: Circumventing D-Bus authentication mechanisms
- **Authorization issues**: Accessing services or methods without proper permissions
- **Memory safety violations**: Use-after-free, buffer overflows, or other memory corruption
  issues
- **Deserialization vulnerabilities**: Malicious input causing crashes, memory corruption, or
  code execution
- **Denial of service**: Maliciously crafted messages causing excessive resource consumption
- **Information disclosure**: Unintended exposure of sensitive data through D-Bus communication
- **Message injection**: Ability to inject or modify D-Bus messages in transit
- **Privilege escalation**: Gaining elevated privileges through D-Bus communication

## Response Timeline

We are committed to responding to security reports promptly:

- **Acknowledgment**: We will acknowledge receipt of your vulnerability report within
  **48 hours**
- **Initial assessment**: We will provide an initial assessment of the report within
  **5 business days**
- **Regular updates**: We will provide progress updates at least every **7 days** until
  resolution
- **Resolution**: We aim to provide a fix or mitigation within **30 days** for critical
  vulnerabilities

Response times may vary based on the complexity of the issue and availability of maintainers.

## Disclosure Policy

We follow a coordinated disclosure process:

1. **Private disclosure**: We will work with you to understand and validate the vulnerability
2. **Fix development**: We will develop and test a fix in a private repository if necessary
3. **Coordinated release**: We will coordinate the public disclosure with the release of a fix
4. **Public disclosure**: After a fix is available, we will publish a security advisory

We request that you:
- Give us reasonable time to address the vulnerability before making it public
- Avoid accessing or modifying data beyond what is necessary to demonstrate the vulnerability
- Act in good faith and avoid privacy violations or destructive behavior

## Security Advisories

Published security advisories will be available through:

- GitHub Security Advisories on the
  [zbus repository](https://github.com/dbus2/zbus/security/advisories)
- [RustSec Advisory Database](https://rustsec.org/)
- Release notes and changelog entries

## Recognition

We appreciate the security research community's efforts to improve the security of zbus. With
your permission, we will acknowledge your contribution in:

- Security advisories
- Release notes
- Project documentation

If you prefer to remain anonymous, please let us know in your report.

## Scope

This security policy covers all repositories and crates maintained by the dbus2 organization:

- **zbus**: Main D-Bus API and connection handling
- **zvariant**: D-Bus/GVariant serialization library
- **zbus_names**: Type-safe D-Bus name handling
- **zbus_macros**: Procedural macros for interfaces and proxies
- **zbus_xml**: D-Bus introspection XML handling
- **zbus_xmlgen**: Code generation from D-Bus interface XML
- **zvariant_derive**: Derive macros for zvariant
- **zvariant_utils**: Utilities for zvariant

## Additional Resources

- [Contributing Guidelines](CONTRIBUTING.md)
- [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct)
- [Rust Security Policy](https://www.rust-lang.org/policies/security)

Thank you for helping to keep zbus and the Rust ecosystem secure!
