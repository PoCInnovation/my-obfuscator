> If you see this section, you've just created a repository using [PoC Innovation's Open-Source project template](https://github.com/PoCInnovation/open-source-project-template). Check the [getting started guide](./.github/getting-started.md).

# my-obfuscator

A python obfucator written in rust using tree-sitter and tree-sitter-python

## How does it work?

The obfucation methods used aim at obfucating the code in-place rather than having some sort of exec block doing all the work

## Getting Started

### Installation

Install the Rust programming language (see instructions at <https://rustup.rs>).

### Quickstart

Clone the repo, and launch the following command:

```bash
cargo run -- -h
```

### Usage

```bash
cargo run -- <PYTHON_SCRIPT_PATH> [OPTIONS]
```
You may use ``` -s int string fn bools dead rm_cmt call ```
with each one being an optional obfuscation.
int will do the oposite of constant folding for integers
string will replace each character with an hexadecimal escape sequence
fn will change the name of the function as well as their calls to random identifiers
bools will attempt to hide the use of boolean values through a few means
dead will insert unreachable code through branching
rm_cmt will remove comments and empty lines
call will hide function calls using eval, it is off by default as it conflicts with fn and it is more expensive, the two could be made compatible but it is unlikely to actually be done now.

## Get involved

You're invited to join this project ! Check out the [contributing guide](./CONTRIBUTING.md).

If you're interested in how the project is organized at a higher level, please contact the current project manager.

## Our PoC team ❤️

Developers
| [<img src="https://github.com/MrZalTy.png?size=85" width=85><br><sub>[Developer's name]</sub>](https://github.com/MrZalTy) | [<img src="https://github.com/MrZalTy.png?size=85" width=85><br><sub>[Developer's name]</sub>](https://github.com/MrZalTy) | [<img src="https://github.com/MrZalTy.png?size=85" width=85><br><sub>[Developer's name]</sub>](https://github.com/MrZalTy)
| :---: | :---: | :---: |

Manager
| [<img src="https://github.com/adrienfort.png?size=85" width=85><br><sub>[Manager's name]</sub>](https://github.com/adrienfort)
| :---: |

<h2 align=center>
Organization
</h2>

<p align='center'>
    <a href="https://www.linkedin.com/company/pocinnovation/mycompany/">
        <img src="https://img.shields.io/badge/LinkedIn-0077B5?style=for-the-badge&logo=linkedin&logoColor=white" alt="LinkedIn logo">
    </a>
    <a href="https://www.instagram.com/pocinnovation/">
        <img src="https://img.shields.io/badge/Instagram-E4405F?style=for-the-badge&logo=instagram&logoColor=white" alt="Instagram logo"
>
    </a>
    <a href="https://twitter.com/PoCInnovation">
        <img src="https://img.shields.io/badge/Twitter-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white" alt="Twitter logo">
    </a>
    <a href="https://discord.com/invite/Yqq2ADGDS7">
        <img src="https://img.shields.io/badge/Discord-7289DA?style=for-the-badge&logo=discord&logoColor=white" alt="Discord logo">
    </a>
</p>
<p align=center>
    <a href="https://www.poc-innovation.fr/">
        <img src="https://img.shields.io/badge/WebSite-1a2b6d?style=for-the-badge&logo=GitHub Sponsors&logoColor=white" alt="Website logo">
    </a>
</p>

> 🚀 Don't hesitate to follow us on our different networks, and put a star 🌟 on `PoC's` repositories

> Made with ❤️ by PoC
