# Super-Mega Template

This is a general-purpose template for XeLaTeX.
You can use it as a foundation for reports, articles, etc.

## What's Included

The template provides the minimal boilerplate for:
* working with:
  * figures
  * tables
  * listings (with ligatures & syntax highlighting)
* pages with arbitrary layout
* the table of contents (ToC)
* the list of:
  * figures
  * tables
  * listings
* the bibliography
* clickable references for the above

## Project Structure

To make the whole thing clear and intuitive, the whole project is organized in "modules".

`main.tex` is a general name for "the main" file of a module. Typically, such a file makes use of other files of the module, making it the only thing the outer world needs. But this is not a strict rule, rather a recommendation.

For example, the `settings` folder is a collection of various things needed to configure the project, but the `settings/packages` is a module that configures external dependencies (imports packages, configures the defaults and defines common topic-related helpers). Trivial imports may go directly in `settings/packages/main.tex`.

The `settings/core.tex` file contains some vital global things that may be needed by the project itself (like the `\append` macro).

## How to Compile

The particular command I use to compile the project is:

```
xelatex.exe -synctex=0 -interaction=nonstopmode --shell-escape --output-directory=output %.tex
```

If using TeXstudio, this is what should be written to the XeLaTeX command text field. Then in the compilation tab the default compiler should be `txs:///xelatex`, and the compilation and preview field should contain `txs:///compile | txs:///view-pdf`. The PDF viewer is configured as follows: `txs:///view-pdf-internal --embedded`. The compilation then can be started by pressing `F5`.

Since the project uses Biber as the BibLaTeX backend, it should be run as:

```
biber.exe --output_directory output %
```
In TeXstudio's compilation tab, the default bibliography must be `txs:///biber`. To recompile the bibliography, I use: `txs:///compile | txs:///bibliography | txs:///compile`. The bibliography can be recompiled by pressing `F8`.

When compiling a project, the workflow is:

1. Compile the project the first time.
1. Compile the bibliography.
    - The bibliography checks what entries are really used, and inserts them only.
1. Compile the project the second time.
    - The document is being populated with references to the bibliography entries, but they all get numbered as 0.
1. Compile the project the third time.
    - Fixes the numbering of the entries.
