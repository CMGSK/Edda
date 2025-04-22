# Contributing to Edda

First off, thanks for taking the time to contribute!

All types of contributions are encouraged and valued. See the [Table of Contents](#table-of-contents) for different ways to help and details about how this project handles them. Please make sure to read the relevant section before making your contribution. It will make it a lot easier for us maintainers and smooth out the experience for all involved. The community looks forward to your contributions. 🎉

> And if you like the project, but just don't have time to contribute, that's fine. There are other easy ways to support the project and show your appreciation, which we would also be very happy about:
> - Star the project
> - Tweet about it
> - Mention the project to your friends/colleagues


## Table of Contents

- [I Have a Question](#i-have-a-question)
  - [I Want To Contribute](#i-want-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Your First Code Contribution](#your-first-code-contribution)
  - [Improving The Documentation](#improving-the-documentation)
- [Styleguides](#styleguides)
- [Join The Project Team](#join-the-project-team)


## I Have a Question

Before you ask a question, it is best to search for existing [Issues](https://github.com/CMGSK/Edda/issues) that might help you. In case you have found a suitable issue and still need clarification, you can write your question in this issue. It is also advisable to search the internet for answers first.

If you then still feel the need to ask a question and need clarification, we recommend the following:

- Open an [Issue](https://github.com/CMGSK/Edda/issues/new).
- Provide as much context as you can about what you're running into.

We will then take care of the issue as soon as possible.


## I Want To Contribute

> ### Legal Notice
> When contributing to this project, you must agree that you have authored 100% of the content, that you have the necessary rights to the content and that the content you contribute may be provided under the project licence.


### Reporting Bugs

#### Before Submitting a Bug Report

A good bug report shouldn't leave others needing to chase you up for more information. Therefore, we ask you to investigate carefully, collect information and describe the issue in detail in your report. Please complete the following steps in advance to help us fix any potential bug as fast as possible.

- Make sure that you are using the latest version.
- Determine if your bug is really a bug and not an error on your side e.g. using incompatible environment components/versions. If you are looking for support, you might want to check [this section](#i-have-a-question)).
- To see if other users have experienced (and potentially already solved) the same issue you are having, check if there is not already a bug report existing for your bug or error in the [bug tracker](https://github.com/CMGSK/Edda/issues?q=label%3Abug).
- Also make sure to search the internet (including Stack Overflow) to see if users outside of the GitHub community have discussed the issue.
- Collect information about the bug:
  - Stack trace (Traceback)
  - OS, Platform and Version (Windows, Linux, macOS, x86, ARM)
  - Version of the interpreter, compiler, SDK, runtime environment, package manager, depending on what seems relevant.
  - Possibly your input and the output
  - Can you reliably reproduce the issue? And can you also reproduce it with older versions?


#### How Do I Submit a Good Bug Report?

> You must never report security related issues, vulnerabilities or bugs including sensitive information to the issue tracker, or elsewhere in public. Instead sensitive bugs must be sent by email to <carlosmgov7k@gmail.com>.

We use GitHub issues to track bugs and errors. If you run into an issue with the project:

- Open an [Issue](https://github.com/CMGSK/Edda/issues/new). (Since we can't be sure at this point whether it is a bug or not, we ask you not to talk about a bug yet and not to label the issue.)
- Explain the behavior you would expect and the actual behavior.
- Please provide as much context as possible and describe the *reproduction steps* that someone else can follow to recreate the issue on their own. This usually includes your code. For good bug reports you should isolate the problem and create a reduced test case.
- Provide the information you collected in the previous section.

Once it's filed:

- The project team will label the issue accordingly.
- A team member will try to reproduce the issue with your provided steps. If there are no reproduction steps or no obvious way to reproduce the issue, the team will ask you for those steps and mark the issue as `needs-repro`. Bugs with the `needs-repro` tag will not be addressed until they are reproduced.
- If the team is able to reproduce the issue, it will be marked `needs-fix`, as well as possibly other tags (such as `critical`), and the issue will be left to be [implemented by someone](#your-first-code-contribution).


### Your First Code Contribution

We assume you're a relatively curated developer, so we are not going to explain the basics here. Right now, everyone is welcome to contribute to the project, since it's lacking a lot of the targeted implementations. The only rules that apply are:

- Only Open Source technologies. Preference for MIT, BSD, GLP... I don't want any weird heavily commercialised dependency, even if licensing allows us.
- The code has to be formatted. We're using cargo fmt (it is included in Rust Analyzer if you're using it, or you can install it manually).
- The branches must follow a consistent pattern. We usually indicate whether is a fix, an improvement or a feature, separated by a slash (`/`) and a brief description separated by dashes (`-`).
- If you're contributing with a new feature, it's a minimum to have the happy paths covered with tests.
- Please write meaningful commit messages.

These principles also apply to improving the documentation, CI pipelines, themes, and everything in general.


## Styleguides

> KISS and DRY.

No really, we're not looking for either "it just works" nor blazing fast if that implies the code is unreadable. We may change things along the way since we're all learning with this project. So I'll just leave here a few mentions of what i expect:

- Free commit messages as long as they're meaningful
- Branches should look like: (fix|improvement|feature):brief-description-dash-separated
- Keep it well structured. If something has the potential to be a library, build a library. If something is out of place, move it where it should belong. We can discuss these things, maybe one of us learns something new.
- Let's aim to have a clear, minimalistic appareance, in semi-light (soon enough) colors and implement all the necessary fundamentals properly. We can move on with more complex things once we're there.
- Keep it SO agnostic. I don't want any feature that cannot operate in a different Operative System. If the logic has to be separated, so be it, but please no halfway features.


## Join The Project Team

We're open to receive new members for the project and make sure it becomes something. Hit me up if you're really interested!.
