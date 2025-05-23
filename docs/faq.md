# FAQ

## Is the Ruff linter compatible with Black?

Yes. The Ruff linter is compatible with [Black](https://github.com/psf/black) out-of-the-box, as
long as the [`line-length`](settings.md#line-length) setting is consistent between the two.

Ruff is designed to be used alongside a formatter (like Ruff's own formatter, or Black) and, as
such, will defer implementing stylistic rules that are obviated by automated formatting.

Note that Ruff's linter and Black treat line-length enforcement a little differently. Black, like
Ruff's formatter, makes a best-effort attempt to adhere to the
[`line-length`](settings.md#line-length), but avoids automatic line-wrapping in some cases (e.g.,
within comments). Ruff, on the other hand, will flag [`line-too-long`](rules/line-too-long.md)
(`E501`) for any line that exceeds the [`line-length`](settings.md#line-length) setting. As such, if
[`line-too-long`](rules/line-too-long.md) (`E501`) is enabled, Ruff can still trigger line-length
violations even when Black or `ruff format` is enabled.

## How does Ruff's formatter compare to Black?

The Ruff formatter is designed to be a drop-in replacement for [Black](https://github.com/psf/black).

Specifically, the formatter is intended to emit near-identical output when run over Black-formatted
code. When run over extensive Black-formatted projects like Django and Zulip, > 99.9% of lines
are formatted identically. When migrating an existing project from Black to Ruff, you should expect
to see a few differences on the margins, but the vast majority of your code should be unchanged.

When run over _non_-Black-formatted code, the formatter makes some different decisions than Black,
and so more deviations should be expected, especially around the treatment of end-of-line comments.

See [_Style Guide_](formatter.md#style-guide) for more.

## How does Ruff's linter compare to Flake8?

Ruff can be used as a drop-in replacement for Flake8 when used (1) without or with a small number of
plugins, (2) alongside Black, and (3) on Python 3 code.

Under those conditions, Ruff implements every rule in Flake8. In practice, that means Ruff
implements all of the `F` rules (which originate from Pyflakes), along with a subset of the `E` and
`W` rules (which originate from pycodestyle).

Ruff also re-implements some of the most popular Flake8 plugins and related code quality tools
natively, including:

- [autoflake](https://pypi.org/project/autoflake/)
- [eradicate](https://pypi.org/project/eradicate/)
- [flake8-2020](https://pypi.org/project/flake8-2020/)
- [flake8-annotations](https://pypi.org/project/flake8-annotations/)
- [flake8-async](https://pypi.org/project/flake8-async)
- [flake8-bandit](https://pypi.org/project/flake8-bandit/) ([#1646](https://github.com/astral-sh/ruff/issues/1646))
- [flake8-blind-except](https://pypi.org/project/flake8-blind-except/)
- [flake8-boolean-trap](https://pypi.org/project/flake8-boolean-trap/)
- [flake8-bugbear](https://pypi.org/project/flake8-bugbear/)
- [flake8-builtins](https://pypi.org/project/flake8-builtins/)
- [flake8-commas](https://pypi.org/project/flake8-commas/)
- [flake8-comprehensions](https://pypi.org/project/flake8-comprehensions/)
- [flake8-copyright](https://pypi.org/project/flake8-copyright/)
- [flake8-datetimez](https://pypi.org/project/flake8-datetimez/)
- [flake8-debugger](https://pypi.org/project/flake8-debugger/)
- [flake8-django](https://pypi.org/project/flake8-django/)
- [flake8-docstrings](https://pypi.org/project/flake8-docstrings/)
- [flake8-eradicate](https://pypi.org/project/flake8-eradicate/)
- [flake8-errmsg](https://pypi.org/project/flake8-errmsg/)
- [flake8-executable](https://pypi.org/project/flake8-executable/)
- [flake8-gettext](https://pypi.org/project/flake8-gettext/)
- [flake8-implicit-str-concat](https://pypi.org/project/flake8-implicit-str-concat/)
- [flake8-import-conventions](https://pypi.org/project/flake8-import-conventions/)
- [flake8-logging](https://pypi.org/project/flake8-logging-format/)
- [flake8-logging-format](https://pypi.org/project/flake8-logging-format/)
- [flake8-no-pep420](https://pypi.org/project/flake8-no-pep420)
- [flake8-pie](https://pypi.org/project/flake8-pie/)
- [flake8-print](https://pypi.org/project/flake8-print/)
- [flake8-pyi](https://pypi.org/project/flake8-pyi/)
- [flake8-pytest-style](https://pypi.org/project/flake8-pytest-style/)
- [flake8-quotes](https://pypi.org/project/flake8-quotes/)
- [flake8-raise](https://pypi.org/project/flake8-raise/)
- [flake8-return](https://pypi.org/project/flake8-return/)
- [flake8-self](https://pypi.org/project/flake8-self/)
- [flake8-simplify](https://pypi.org/project/flake8-simplify/)
- [flake8-slots](https://pypi.org/project/flake8-slots/)
- [flake8-super](https://pypi.org/project/flake8-super/)
- [flake8-tidy-imports](https://pypi.org/project/flake8-tidy-imports/)
- [flake8-todos](https://pypi.org/project/flake8-todos/)
- [flake8-type-checking](https://pypi.org/project/flake8-type-checking/)
- [flake8-use-pathlib](https://pypi.org/project/flake8-use-pathlib/)
- [flynt](https://pypi.org/project/flynt/) ([#2102](https://github.com/astral-sh/ruff/issues/2102))
- [isort](https://pypi.org/project/isort/)
- [mccabe](https://pypi.org/project/mccabe/)
- [pandas-vet](https://pypi.org/project/pandas-vet/)
- [pep8-naming](https://pypi.org/project/pep8-naming/)
- [perflint](https://pypi.org/project/perflint/) ([#4789](https://github.com/astral-sh/ruff/issues/4789))
- [pydocstyle](https://pypi.org/project/pydocstyle/)
- [pygrep-hooks](https://github.com/pre-commit/pygrep-hooks)
- [pyupgrade](https://pypi.org/project/pyupgrade/)
- [tryceratops](https://pypi.org/project/tryceratops/)
- [yesqa](https://pypi.org/project/yesqa/)

Note that, in some cases, Ruff uses different rule codes and prefixes than would be found in the
originating Flake8 plugins. For example, Ruff uses `TID252` to represent the `I252` rule from
flake8-tidy-imports. This helps minimize conflicts across plugins and allows any individual plugin
to be toggled on or off with a single (e.g.) `--select TID`, as opposed to `--select I2` (to avoid
conflicts with the isort rules, like `I001`).

Beyond the rule set, Ruff's primary limitation vis-à-vis Flake8 is that it does not support custom
lint rules. (Instead, popular Flake8 plugins are re-implemented in Rust as part of Ruff itself.)
One minor difference is that Ruff doesn't include all the 'opinionated' rules from flake8-bugbear.

## How does Ruff's linter compare to Pylint?

At time of writing, Pylint implements ~409 total rules, while Ruff implements over 800, of which at
least 209 overlap with the Pylint rule set (see: [#970](https://github.com/astral-sh/ruff/issues/970)).

Pylint implements many rules that Ruff does not, and vice versa. For example, Pylint does more type
inference than Ruff (e.g., Pylint can validate the number of arguments in a function call). As such,
Ruff is not a "pure" drop-in replacement for Pylint (and vice versa), as they enforce different sets
of rules.

Despite these differences, many users have successfully switched from Pylint to Ruff, especially
those using Ruff alongside a [type checker](faq.md#how-does-ruff-compare-to-mypy-or-pyright-or-pyre),
which can cover some of the functionality that Pylint provides.

Like Flake8, Pylint supports plugins (called "checkers"), while Ruff implements all rules natively
and does not support custom or third-party rules. Unlike Pylint, Ruff is capable of automatically
fixing its own lint violations.

In some cases, Ruff's rules may yield slightly different results than their Pylint counterparts. For
example, Ruff's [`too-many-branches`](rules/too-many-branches.md) does not count `try` blocks as
their own branches, unlike Pylint's `R0912`. Ruff's `PL` rule group also includes a small number of
rules from Pylint _extensions_ (like [`magic-value-comparison`](rules/magic-value-comparison.md)),
which need to be explicitly activated when using Pylint. By enabling Ruff's `PL` group, you may
see violations for rules that weren't previously enabled through your Pylint configuration.

Pylint parity is being tracked in [#970](https://github.com/astral-sh/ruff/issues/970).

## How does Ruff compare to Mypy, or Pyright, or Pyre?

Ruff is a linter, not a type checker. It can detect some of the same problems that a type checker
can, but a type checker will catch certain errors that Ruff would miss. The opposite is also true:
Ruff will catch certain errors that a type checker would typically ignore.

For example, unlike a type checker, Ruff will notify you if an import is unused, by looking for
references to that import in the source code; on the other hand, a type checker could flag that you
passed an integer argument to a function that expects a string, which Ruff would miss. The
tools are complementary.

It's recommended that you use Ruff in conjunction with a type checker, like Mypy, Pyright, or Pyre,
with Ruff providing faster feedback on lint violations and the type checker providing more detailed
feedback on type errors.

## Which tools does Ruff replace?

Today, Ruff can be used to replace Flake8 when used with any of the following plugins:

- [flake8-2020](https://pypi.org/project/flake8-2020/)
- [flake8-annotations](https://pypi.org/project/flake8-annotations/)
- [flake8-async](https://pypi.org/project/flake8-async)
- [flake8-bandit](https://pypi.org/project/flake8-bandit/) ([#1646](https://github.com/astral-sh/ruff/issues/1646))
- [flake8-blind-except](https://pypi.org/project/flake8-blind-except/)
- [flake8-boolean-trap](https://pypi.org/project/flake8-boolean-trap/)
- [flake8-bugbear](https://pypi.org/project/flake8-bugbear/)
- [flake8-builtins](https://pypi.org/project/flake8-builtins/)
- [flake8-commas](https://pypi.org/project/flake8-commas/)
- [flake8-comprehensions](https://pypi.org/project/flake8-comprehensions/)
- [flake8-copyright](https://pypi.org/project/flake8-copyright/)
- [flake8-datetimez](https://pypi.org/project/flake8-datetimez/)
- [flake8-debugger](https://pypi.org/project/flake8-debugger/)
- [flake8-django](https://pypi.org/project/flake8-django/)
- [flake8-docstrings](https://pypi.org/project/flake8-docstrings/)
- [flake8-eradicate](https://pypi.org/project/flake8-eradicate/)
- [flake8-errmsg](https://pypi.org/project/flake8-errmsg/)
- [flake8-executable](https://pypi.org/project/flake8-executable/)
- [flake8-gettext](https://pypi.org/project/flake8-gettext/)
- [flake8-implicit-str-concat](https://pypi.org/project/flake8-implicit-str-concat/)
- [flake8-import-conventions](https://pypi.org/project/flake8-import-conventions/)
- [flake8-logging](https://pypi.org/project/flake8-logging/)
- [flake8-logging-format](https://pypi.org/project/flake8-logging-format/)
- [flake8-no-pep420](https://pypi.org/project/flake8-no-pep420)
- [flake8-pie](https://pypi.org/project/flake8-pie/)
- [flake8-print](https://pypi.org/project/flake8-print/)
- [flake8-pytest-style](https://pypi.org/project/flake8-pytest-style/)
- [flake8-quotes](https://pypi.org/project/flake8-quotes/)
- [flake8-raise](https://pypi.org/project/flake8-raise/)
- [flake8-return](https://pypi.org/project/flake8-return/)
- [flake8-self](https://pypi.org/project/flake8-self/)
- [flake8-simplify](https://pypi.org/project/flake8-simplify/)
- [flake8-slots](https://pypi.org/project/flake8-slots/)
- [flake8-super](https://pypi.org/project/flake8-super/)
- [flake8-tidy-imports](https://pypi.org/project/flake8-tidy-imports/)
- [flake8-todos](https://pypi.org/project/flake8-todos/)
- [flake8-type-checking](https://pypi.org/project/flake8-type-checking/)
- [flake8-use-pathlib](https://pypi.org/project/flake8-use-pathlib/)
- [flynt](https://pypi.org/project/flynt/) ([#2102](https://github.com/astral-sh/ruff/issues/2102))
- [mccabe](https://pypi.org/project/mccabe/)
- [pandas-vet](https://pypi.org/project/pandas-vet/)
- [pep8-naming](https://pypi.org/project/pep8-naming/)
- [perflint](https://pypi.org/project/perflint/) ([#4789](https://github.com/astral-sh/ruff/issues/4789))
- [pydocstyle](https://pypi.org/project/pydocstyle/)
- [tryceratops](https://pypi.org/project/tryceratops/)

Ruff can also replace [Black](https://pypi.org/project/black/), [isort](https://pypi.org/project/isort/),
[yesqa](https://pypi.org/project/yesqa/), [eradicate](https://pypi.org/project/eradicate/), and
most of the rules implemented in [pyupgrade](https://pypi.org/project/pyupgrade/).

If you're looking to use Ruff, but rely on an unsupported Flake8 plugin, feel free to file an
[issue](https://github.com/astral-sh/ruff/issues/new).

## Do I have to use Ruff's linter and formatter together?

Nope! Ruff's linter and formatter can be used independently of one another -- you can use
Ruff as a formatter, but not a linter, or vice versa.

## What versions of Python does Ruff support?

Ruff can lint code for any Python version from 3.7 onwards, including Python 3.13.

Ruff does not support Python 2. Ruff _may_ run on pre-Python 3.7 code, although such versions
are not officially supported (e.g., Ruff does _not_ respect type comments).

Ruff is installable under any Python version from 3.7 onwards.

## Do I need to install Rust to use Ruff?

Nope! Ruff is available as [`ruff`](https://pypi.org/project/ruff/) on PyPI. We recommend installing Ruff with [uv](https://docs.astral.sh/uv/),
though it's also installable with `pip`, `pipx`, and a [variety of other package managers](installation.md):

```console
$ # Install Ruff globally.
$ uv tool install ruff@latest

$ # Or add Ruff to your project.
$ uv add --dev ruff

$ # With pip.
$ pip install ruff

$ # With pipx.
$ pipx install ruff
```

Starting with version `0.5.0`, Ruff can also be installed with our standalone installers:

```console
$ # On macOS and Linux.
$ curl -LsSf https://astral.sh/ruff/install.sh | sh

$ # On Windows.
$ powershell -c "irm https://astral.sh/ruff/install.ps1 | iex"

$ # For a specific version.
$ curl -LsSf https://astral.sh/ruff/0.5.0/install.sh | sh
$ powershell -c "irm https://astral.sh/ruff/0.5.0/install.ps1 | iex"
```

Ruff ships with wheels for all major platforms, which enables `uv`, `pip`, and other tools to install Ruff without
relying on a Rust toolchain at all.

## Can I write my own linter plugins for Ruff?

Ruff does not yet support third-party plugins, though a plugin system is within-scope for the
project. See [#283](https://github.com/astral-sh/ruff/issues/283) for more.

## How does Ruff's import sorting compare to [isort](https://pypi.org/project/isort/)?

Ruff's import sorting is intended to be near-equivalent to isort's when using isort's
`profile = "black"`.

There are a few known differences in how Ruff and isort treat aliased imports, and in how Ruff and
isort treat inline comments in some cases (see: [#1381](https://github.com/astral-sh/ruff/issues/1381),
[#2104](https://github.com/astral-sh/ruff/issues/2104)).

For example, Ruff tends to group non-aliased imports from the same module:

```python
from numpy import cos, int8, int16, int32, int64, tan, uint8, uint16, uint32, uint64
from numpy import sin as np_sin
```

Whereas isort splits them into separate import statements at each aliased boundary:

```python
from numpy import cos, int8, int16, int32, int64
from numpy import sin as np_sin
from numpy import tan, uint8, uint16, uint32, uint64
```

Ruff also correctly classifies some modules as standard-library that aren't recognized
by isort, like `_string` and `idlelib`.

Like isort, Ruff's import sorting is compatible with Black.

## How does Ruff determine which of my imports are first-party, third-party, etc.?

Ruff accepts a `src` option that in your `pyproject.toml`, `ruff.toml`, or `.ruff.toml` file,
specifies the directories that Ruff should consider when determining whether an import is
first-party.

For example, if you have a project with the following structure:

```tree
my_project
├── pyproject.toml
└── src
    └── foo
        ├── __init__.py
        └── bar
            ├── __init__.py
            └── baz.py
```

When Ruff sees an import like `import foo`, it will then iterate over the `src` directories,
looking for a corresponding Python module (in reality, a directory named `foo` or a file named
`foo.py`). For module paths with multiple components like `import foo.bar`,
the default behavior is to search only for a directory named `foo` or a file
named `foo.py`. However, if `preview` is enabled, Ruff will require that the full relative path `foo/bar` exists as a directory, or that `foo/bar.py` or `foo/bar.pyi` exist as files. Finally, imports of the form `from foo import bar`, Ruff will only use `foo` when determining whether a module is first-party or third-party. 

If there is a directory
whose name matches a third-party package, but does not contain Python code,
it could happen that the above algorithm incorrectly infers an import to be first-party.
To prevent this, you can modify the [`known-third-party`](settings.md#lint_isort_known-third-party) setting. For example, if you import
the package `wandb` but also have a subdirectory of your `src` with
the same name, you can add the following:

=== "pyproject.toml"

    ```toml
    [tool.ruff.lint.isort]
    known-third-party = ["wandb"]
    ```

=== "ruff.toml"

    ```toml
    [lint.isort]
    known-third-party = ["wandb"]
    ```


If the `src` field is omitted, Ruff will default to using the "project root", along with a `"src"`
subdirectory, as the first-party sources, to support both flat and nested project layouts.
The "project root" is typically the directory containing your `pyproject.toml`, `ruff.toml`, or
`.ruff.toml` file, unless a configuration file is provided on the command-line via the `--config`
option, in which case, the current working directory is used as the project root.

In this case, Ruff would check the `"src"` directory by default, but we can configure it as an
explicit, exclusive first-party source like so:

=== "pyproject.toml"

    ```toml
    [tool.ruff]
    # Ruff supports a top-level `src` option in lieu of isort's `src_paths` setting.
    # All paths are relative to the project root, which is the directory containing the pyproject.toml.
    src = ["src"]
    ```

=== "ruff.toml"

    ```toml
    # Ruff supports a top-level `src` option in lieu of isort's `src_paths` setting.
    # All paths are relative to the project root, which is the directory containing the pyproject.toml.
    src = ["src"]
    ```

If your `pyproject.toml`, `ruff.toml`, or `.ruff.toml` extends another configuration file, Ruff
will still use the directory containing your `pyproject.toml`, `ruff.toml`, or `.ruff.toml` file as
the project root (as opposed to the directory of the file pointed to via the `extends` option).

For example, if you add a configuration file to the `tests` directory in the above example, you'll
want to explicitly set the `src` option in the extended configuration file:

=== "pyproject.toml"

    ```toml
    [tool.ruff]
    extend = "../pyproject.toml"
    src = ["../src"]
    ```

=== "ruff.toml"

    ```toml
    extend = "../pyproject.toml"
    src = ["../src"]
    ```

Beyond this `src`-based detection, Ruff will also attempt to determine the current Python package
for a given Python file, and mark imports from within the same package as first-party. For example,
above, `baz.py` would be identified as part of the Python package beginning at
`./my_project/src/foo`, and so any imports in `baz.py` that begin with `foo` (like `import foo.bar`)
would be considered first-party based on this same-package heuristic.

For a detailed explanation of `src` resolution, see the [contributing guide](contributing.md).

Ruff can also be configured to treat certain modules as (e.g.) always first-party, regardless of
their location on the filesystem. For example, you can set [`known-first-party`](settings.md#lint_isort_known-first-party)
like so:

=== "pyproject.toml"

    ```toml
    [tool.ruff]
    src = ["src", "tests"]

    [tool.ruff.lint]
    select = [
        # Pyflakes
        "F",
        # Pycodestyle
        "E",
        "W",
        # isort
        "I001"
    ]

    [tool.ruff.lint.isort]
    known-first-party = ["my_module1", "my_module2"]
    ```

=== "ruff.toml"

    ```toml
    src = ["src", "tests"]

    [lint]
    select = [
        # Pyflakes
        "F",
        # Pycodestyle
        "E",
        "W",
        # isort
        "I001"
    ]

    [lint.isort]
    known-first-party = ["my_module1", "my_module2"]
    ```

Ruff does not yet support all of isort's configuration options, though it does support many of
them. You can find the supported settings in the [API reference](settings.md#lintisort).

## Does Ruff support Jupyter Notebooks?

Ruff has built-in support for linting and formatting [Jupyter Notebooks](https://jupyter.org/). Refer to the
[Jupyter Notebook section](configuration.md#jupyter-notebook-discovery) for more details.

Ruff also integrates with [nbQA](https://github.com/nbQA-dev/nbQA), a tool for running linters and
code formatters over Jupyter Notebooks.

After installing `ruff` and `nbqa`, you can run Ruff over a notebook like so:

```console
$ nbqa ruff Untitled.ipynb
Untitled.ipynb:cell_1:2:5: F841 Local variable `x` is assigned to but never used
Untitled.ipynb:cell_2:1:1: E402 Module level import not at top of file
Untitled.ipynb:cell_2:1:8: F401 `os` imported but unused
Found 3 errors.
1 potentially fixable with the --fix option.
```

## Does Ruff support NumPy- or Google-style docstrings?

Yes! To enforce a docstring convention, add a [`convention`](settings.md#lint_pydocstyle_convention)
setting following to your configuration file:

=== "pyproject.toml"

    ```toml
    [tool.ruff.lint.pydocstyle]
    convention = "google"  # Accepts: "google", "numpy", or "pep257".
    ```

=== "ruff.toml"

    ```toml
    [lint.pydocstyle]
    convention = "google"  # Accepts: "google", "numpy", or "pep257".
    ```

For example, if you're coming from flake8-docstrings, and your originating configuration uses
`--docstring-convention=numpy`, you'd instead set `convention = "numpy"` in your `pyproject.toml`,
as above.

Alongside [`convention`](settings.md#lint_pydocstyle_convention), you'll want to
explicitly enable the `D` rule code prefix, since the `D` rules are not enabled by default:

=== "pyproject.toml"

    ```toml
    [tool.ruff.lint]
    select = ["D"]

    [tool.ruff.lint.pydocstyle]
    convention = "google"
    ```

=== "ruff.toml"

    ```toml
    [lint]
    select = ["D"]

    [lint.pydocstyle]
    convention = "google"
    ```

Enabling a [`convention`](settings.md#lint_pydocstyle_convention) will disable any rules that are not
included in the specified convention. As such, the intended workflow is to enable a convention and
then selectively enable or disable any additional rules on top of it:

=== "pyproject.toml"

    ```toml
    [tool.ruff.lint]
    select = [
        "D",
        # Augment the convention by requiring an imperative mood for all docstrings.
        "D401",
    ]

    ignore = [
        # Relax the convention by _not_ requiring documentation for every function parameter.
        "D417",
    ]

    [tool.ruff.lint.pydocstyle]
    convention = "google"
    ```

=== "ruff.toml"

    ```toml
    [lint]
    select = [
        "D",
        # Augment the convention by requiring an imperative mood for all docstrings.
        "D401",
    ]

    ignore = [
        # Relax the convention by _not_ requiring documentation for every function parameter.
        "D417",
    ]

    [lint.pydocstyle]
    convention = "google"
    ```

The PEP 257 convention includes all `D` errors apart from:
[`D203`](rules/incorrect-blank-line-before-class.md),
[`D212`](rules/multi-line-summary-first-line.md),
[`D213`](rules/multi-line-summary-second-line.md),
[`D214`](rules/overindented-section.md),
[`D215`](rules/overindented-section-underline.md),
[`D404`](rules/docstring-starts-with-this.md),
[`D405`](rules/non-capitalized-section-name.md),
[`D406`](rules/missing-new-line-after-section-name.md),
[`D407`](rules/missing-dashed-underline-after-section.md),
[`D408`](rules/missing-section-underline-after-name.md),
[`D409`](rules/mismatched-section-underline-length.md),
[`D410`](rules/no-blank-line-after-section.md),
[`D411`](rules/no-blank-line-before-section.md),
[`D413`](rules/no-blank-line-after-section.md),
[`D415`](rules/missing-terminal-punctuation.md),
[`D416`](rules/missing-section-name-colon.md), and
[`D417`](rules/undocumented-param.md).

The NumPy convention includes all `D` errors apart from:
[`D107`](rules/undocumented-public-init.md),
[`D203`](rules/incorrect-blank-line-before-class.md),
[`D212`](rules/multi-line-summary-first-line.md),
[`D213`](rules/multi-line-summary-second-line.md),
[`D402`](rules/signature-in-docstring.md),
[`D413`](rules/no-blank-line-after-section.md),
[`D415`](rules/missing-terminal-punctuation.md),
[`D416`](rules/missing-section-name-colon.md), and
[`D417`](rules/undocumented-param.md).

The Google convention includes all `D` errors apart from:
[`D203`](rules/incorrect-blank-line-before-class.md),
[`D204`](rules/incorrect-blank-line-after-class.md),
[`D213`](rules/multi-line-summary-second-line.md),
[`D215`](rules/overindented-section-underline.md),
[`D400`](rules/missing-trailing-period.md),
[`D401`](rules/non-imperative-mood.md),
[`D404`](rules/docstring-starts-with-this.md),
[`D406`](rules/missing-new-line-after-section-name.md),
[`D407`](rules/missing-dashed-underline-after-section.md),
[`D408`](rules/missing-section-underline-after-name.md),
[`D409`](rules/mismatched-section-underline-length.md), and
[`D413`](rules/no-blank-line-after-section.md).

By default, no [`convention`](settings.md#lint_pydocstyle_convention) is set, and so the enabled rules
are determined by the [`select`](settings.md#lint_select) setting alone.

## What is "preview"?

Preview enables a collection of newer rules and fixes that are considered experimental or unstable.
See the [preview documentation](preview.md) for more details; or, to see which rules are currently
in preview, visit the [rules reference](rules.md).

## How can I tell what settings Ruff is using to check my code?

Run `ruff check /path/to/code.py --show-settings` to view the resolved settings for a given file.

## I want to use Ruff, but I don't want to use `pyproject.toml`. What are my options?

In lieu of a `pyproject.toml` file, you can use a `ruff.toml` file for configuration. The two
files are functionally equivalent and have an identical schema, with the exception that a `ruff.toml`
file can omit the `[tool.ruff]` section header. For example:

=== "pyproject.toml"

```toml
[tool.ruff]
line-length = 88

[tool.ruff.lint.pydocstyle]
convention = "google"
```

=== "ruff.toml"

```toml
line-length = 88

[lint.pydocstyle]
convention = "google"
```

Ruff doesn't currently support INI files, like `setup.cfg` or `tox.ini`.

## How can I change Ruff's default configuration?

When no configuration file is found, Ruff will look for a user-specific `ruff.toml` file as a
last resort. This behavior is similar to Flake8's `~/.config/flake8`.

On macOS and Linux, Ruff expects that file to be located at `~/.config/ruff/ruff.toml`,
and respects the `XDG_CONFIG_HOME` specification.

On Windows, Ruff expects that file to be located at `~\AppData\Roaming\ruff\ruff.toml`.

!!! note
    Prior to `v0.5.0`, Ruff would read user-specific configuration from
    `~/Library/Application Support/ruff/ruff.toml` on macOS. While Ruff will still respect
    such configuration files, the use of `~/Library/Application Support` is considered deprecated.

For more, see the [`etcetera`](https://crates.io/crates/etcetera) crate.

## Ruff tried to fix something — but it broke my code. What's going on?

Ruff labels fixes as "safe" and "unsafe". By default, Ruff will fix all violations for which safe
fixes are available, while unsafe fixes can be enabled via the [`unsafe-fixes`](settings.md#unsafe-fixes)
setting, or passing the [`--unsafe-fixes`](settings.md#unsafe-fixes) flag to `ruff check`. For
more, see [the fix documentation](linter.md#fixes).

Even still, given the dynamic nature of Python, it's difficult to have _complete_ certainty when
making changes to code, even for seemingly trivial fixes. If a "safe" fix breaks your code, please
[file an Issue](https://github.com/astral-sh/ruff/issues/new).

## How can I disable/force Ruff's color output?

Ruff's color output is powered by the [`colored`](https://crates.io/crates/colored) crate, which
attempts to automatically detect whether the output stream supports color. However, you can force
colors off by setting the `NO_COLOR` environment variable to any value (e.g., `NO_COLOR=1`), or
force colors on by setting `FORCE_COLOR` to any non-empty value (e.g., `FORCE_COLOR=1`).

[`colored`](https://crates.io/crates/colored) also supports the `CLICOLOR` and `CLICOLOR_FORCE`
environment variables (see the [spec](https://bixense.com/clicolors/)).

## Ruff behaves unexpectedly when using `source.*` code actions in Notebooks. What's going on? {: #source-code-actions-in-notebooks }

Ruff does not support `source.organizeImports` and `source.fixAll` code actions in Jupyter Notebooks
(`notebook.codeActionsOnSave` in VS Code). It's recommended to use the `notebook` prefixed code
actions for the same such as `notebook.source.organizeImports` and `notebook.source.fixAll`
respectively.

Ruff requires to have a full view of the notebook to provide accurate diagnostics and fixes. For
example, if you have a cell that imports a module and another cell that uses that module, Ruff
needs to see both cells to mark the import as used. If Ruff were to only see one cell at a time,
it would incorrectly mark the import as unused.

When using the `source.*` code actions for a Notebook, Ruff will be asked to fix any issues for each
cell in parallel, which can lead to unexpected behavior. For example, if a user has configured to
run `source.organizeImports` code action on save for a Notebook, Ruff will attempt to fix the
imports for the entire notebook corresponding to each cell. This leads to the client making the same
changes to the notebook multiple times, which can lead to unexpected behavior
([astral-sh/ruff-vscode#680](https://github.com/astral-sh/ruff-vscode/issues/680),
[astral-sh/ruff-vscode#640](https://github.com/astral-sh/ruff-vscode/issues/640),
[astral-sh/ruff-vscode#391](https://github.com/astral-sh/ruff-vscode/issues/391)).
