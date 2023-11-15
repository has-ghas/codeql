# PHP analysis support for CodeQL

_NOTE:  *CodeQL does not currently support PHP.* The contents of this directory
        are intended for experimentation and testing ONLY. Builds for PHP are
        self-contained and will remain excluded from any customer-facing build._

This directory contains the extractor, CodeQL libraries, and queries for
*experimental PHP language support* in CodeQL.

It contains two major components:
  1. static analysis libraries and queries written in
     [CodeQL](https://codeql.github.com/docs/) that can be used to analyze such
     a database to find coding mistakes or security vulnerabilities.
  2. an extractor, written in Rust, that parses PHP source code and converts it
     into a database that can be queried using CodeQL. See [Developer
     information](doc/HOWTO.md) for information on building the extractor (you
     do not need to do this if you are only developing queries).

