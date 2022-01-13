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
