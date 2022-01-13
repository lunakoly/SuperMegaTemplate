# Super-Mega Polytech Template

This is a template for university reports in Russian (XeLaTeX).
You can use it as a foundation for reports, articles, etc. requiring GOST formatting.

This repo is based on the general-purpose [Super-Mega Template](https://github.com/lunakoly/SuperMegaTemplate).

## What's Changed

The idea was to override styling on top of the default look and feel. So, all defaults are modified via the `theme` module, rather than changing something in the `settings` directly.

Some things are configured via variables that may optionally be defined before loading the `settings/packages`: these are configured by the `theme/variables.tex`.

Others are just the additional configurations performed after the `settings/packages`: `theme/main.tex`. The `theme` _may_ load additional packages if needed.

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
