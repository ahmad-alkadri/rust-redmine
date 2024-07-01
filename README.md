# Rust-Redmine

A Rust client for interacting with the Redmine REST API. This library
provides a native Rust interface to access and manipulate Redmine
resources, including issues, projects, trackers, and more.

## Features

The following are a list of features available through Redmine's official
REST API and their current state of implementations in this package (on the
current `dev` branch at the moment):

| Features            | Official Redmine State | Implements |
| ------------------- | ---------------------: | ---------: |
| Issues              |                 Stable |       100% |
| Projects            |                 Stable |        50% |
| Project Memberships |                  Alpha |         0% |
| Users               |                 Stable |         0% |
| Time Entries        |                 Stable |         0% |
| News                |              Prorotype |         0% |
| Issue Relations     |                  Alpha |         0% |
| Versions            |                  Alpha |         0% |
| Wiki Pages          |                  Alpha |         0% |
| Queries             |                  Alpha |         0% |
| Attachments         |                   Beta |         0% |
| Issue Statuses      |                  Alpha |         0% |
| Trackers            |                  Alpha |         0% |
| Enumerations        |                  Alpha |         0% |
| Issue Categories    |                  Alpha |         0% |
| Roles               |                  Alpha |         0% |
| Groups              |                  Alpha |         0% |

## Development Status

This library is still under heavy development. While it's functional, it's
not yet complete, and the API may change. Absolutely not recommended for
production. Use at your own risk.

## Contributing

Contributions are welcome! If you'd like to help, please:

- Fork the repository
- Create a new branch for your feature or fix
- Write tests and implement your changes
- Submit a pull request
