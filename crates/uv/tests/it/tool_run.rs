use crate::common::{uv_snapshot, TestContext};
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use indoc::indoc;
use uv_fs::copy_dir_all;
use uv_static::EnvVars;

#[test]
fn tool_run_args() {
    let context = TestContext::new("3.12").with_filtered_counts();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    // We treat arguments before the command as uv arguments
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--help")
        .arg("pytest")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    Run a command provided by a Python package

    Usage: uv tool run [OPTIONS] [COMMAND]

    Options:
          --from <FROM>
              Use the given package to provide the command
          --with <WITH>
              Run with the given packages installed
          --with-editable <WITH_EDITABLE>
              Run with the given packages installed in editable mode
          --with-requirements <WITH_REQUIREMENTS>
              Run with all packages listed in the given `requirements.txt` files
      -c, --constraints <CONSTRAINTS>
              Constrain versions using the given requirements files [env: UV_CONSTRAINT=]
          --overrides <OVERRIDES>
              Override versions using the given requirements files [env: UV_OVERRIDE=]
          --isolated
              Run the tool in an isolated virtual environment, ignoring any already-installed tools
          --env-file <ENV_FILE>
              Load environment variables from a `.env` file [env: UV_ENV_FILE=]
          --no-env-file
              Avoid reading environment variables from a `.env` file [env: UV_NO_ENV_FILE=]

    Index options:
          --index <INDEX>
              The URLs to use when resolving dependencies, in addition to the default index [env:
              UV_INDEX=]
          --default-index <DEFAULT_INDEX>
              The URL of the default package index (by default: <https://pypi.org/simple>) [env:
              UV_DEFAULT_INDEX=]
      -i, --index-url <INDEX_URL>
              (Deprecated: use `--default-index` instead) The URL of the Python package index (by
              default: <https://pypi.org/simple>) [env: UV_INDEX_URL=]
          --extra-index-url <EXTRA_INDEX_URL>
              (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to
              `--index-url` [env: UV_EXTRA_INDEX_URL=]
      -f, --find-links <FIND_LINKS>
              Locations to search for candidate distributions, in addition to those found in the
              registry indexes [env: UV_FIND_LINKS=]
          --no-index
              Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and
              those provided via `--find-links`
          --index-strategy <INDEX_STRATEGY>
              The strategy to use when resolving against multiple index URLs [env: UV_INDEX_STRATEGY=]
              [possible values: first-index, unsafe-first-match, unsafe-best-match]
          --keyring-provider <KEYRING_PROVIDER>
              Attempt to use `keyring` for authentication for index URLs [env: UV_KEYRING_PROVIDER=]
              [possible values: disabled, subprocess]

    Resolver options:
      -U, --upgrade
              Allow package upgrades, ignoring pinned versions in any existing output file. Implies
              `--refresh`
      -P, --upgrade-package <UPGRADE_PACKAGE>
              Allow upgrades for a specific package, ignoring pinned versions in any existing output
              file. Implies `--refresh-package`
          --resolution <RESOLUTION>
              The strategy to use when selecting between the different compatible versions for a given
              package requirement [env: UV_RESOLUTION=] [possible values: highest, lowest,
              lowest-direct]
          --prerelease <PRERELEASE>
              The strategy to use when considering pre-release versions [env: UV_PRERELEASE=] [possible
              values: disallow, allow, if-necessary, explicit, if-necessary-or-explicit]
          --fork-strategy <FORK_STRATEGY>
              The strategy to use when selecting multiple versions of a given package across Python
              versions and platforms [env: UV_FORK_STRATEGY=] [possible values: fewest, requires-python]
          --exclude-newer <EXCLUDE_NEWER>
              Limit candidate packages to those that were uploaded prior to the given date [env:
              UV_EXCLUDE_NEWER=2024-03-25T00:00:00Z]
          --no-sources
              Ignore the `tool.uv.sources` table when resolving dependencies. Used to lock against the
              standards-compliant, publishable package metadata, as opposed to using any workspace, Git,
              URL, or local path sources

    Installer options:
          --reinstall
              Reinstall all packages, regardless of whether they're already installed. Implies
              `--refresh`
          --reinstall-package <REINSTALL_PACKAGE>
              Reinstall a specific package, regardless of whether it's already installed. Implies
              `--refresh-package`
          --link-mode <LINK_MODE>
              The method to use when installing packages from the global cache [env: UV_LINK_MODE=]
              [possible values: clone, copy, hardlink, symlink]
          --compile-bytecode
              Compile Python files to bytecode after installation [env: UV_COMPILE_BYTECODE=]

    Build options:
      -C, --config-setting <CONFIG_SETTING>
              Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs
          --no-build-isolation
              Disable isolation when building source distributions [env: UV_NO_BUILD_ISOLATION=]
          --no-build-isolation-package <NO_BUILD_ISOLATION_PACKAGE>
              Disable isolation when building source distributions for a specific package
          --no-build
              Don't build source distributions [env: UV_NO_BUILD=]
          --no-build-package <NO_BUILD_PACKAGE>
              Don't build source distributions for a specific package [env: UV_NO_BUILD_PACKAGE=]
          --no-binary
              Don't install pre-built wheels [env: UV_NO_BINARY=]
          --no-binary-package <NO_BINARY_PACKAGE>
              Don't install pre-built wheels for a specific package [env: UV_NO_BINARY_PACKAGE=]

    Cache options:
      -n, --no-cache
              Avoid reading from or writing to the cache, instead using a temporary directory for the
              duration of the operation [env: UV_NO_CACHE=]
          --cache-dir [CACHE_DIR]
              Path to the cache directory [env: UV_CACHE_DIR=]
          --refresh
              Refresh all cached data
          --refresh-package <REFRESH_PACKAGE>
              Refresh cached data for a specific package

    Python options:
      -p, --python <PYTHON>      The Python interpreter to use to build the run environment. [env:
                                 UV_PYTHON=]
          --managed-python       Require use of uv-managed Python versions [env: UV_MANAGED_PYTHON=]
          --no-managed-python    Disable use of uv-managed Python versions [env: UV_NO_MANAGED_PYTHON=]
          --no-python-downloads  Disable automatic downloads of Python. [env:
                                 "UV_PYTHON_DOWNLOADS=never"]

    Global options:
      -q, --quiet...
              Use quiet output
      -v, --verbose...
              Use verbose output
          --color <COLOR_CHOICE>
              Control the use of color in output [possible values: auto, always, never]
          --native-tls
              Whether to load TLS certificates from the platform's native certificate store [env:
              UV_NATIVE_TLS=]
          --offline
              Disable network access [env: UV_OFFLINE=]
          --allow-insecure-host <ALLOW_INSECURE_HOST>
              Allow insecure connections to a host [env: UV_INSECURE_HOST=]
          --no-progress
              Hide all progress outputs [env: UV_NO_PROGRESS=]
          --directory <DIRECTORY>
              Change to the given directory prior to running the command
          --project <PROJECT>
              Run the command within the given project directory
          --config-file <CONFIG_FILE>
              The path to a `uv.toml` file to use for configuration [env: UV_CONFIG_FILE=]
          --no-config
              Avoid discovering configuration files (`pyproject.toml`, `uv.toml`) [env: UV_NO_CONFIG=]
      -h, --help
              Display the concise help for this command

    Use `uvx` as a shortcut for `uv tool run`.

    Use `uv help tool run` for more details.

    ----- stderr -----
    "#);

    // We don't treat arguments after the command as uv arguments
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("pytest")
        .arg("--help")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    usage: pytest [options] [file_or_dir] [file_or_dir] [...]

    positional arguments:
      file_or_dir

    general:
      -k EXPRESSION         Only run tests which match the given substring expression. An expression is
                            a Python evaluatable expression where all names are substring-matched
                            against test names and their parent classes. Example: -k 'test_method or
                            test_other' matches all test functions and classes whose name contains
                            'test_method' or 'test_other', while -k 'not test_method' matches those that
                            don't contain 'test_method' in their names. -k 'not test_method and not
                            test_other' will eliminate the matches. Additionally keywords are matched to
                            classes and functions containing extra names in their
                            'extra_keyword_matches' set, as well as functions which have names assigned
                            directly to them. The matching is case-insensitive.
      -m MARKEXPR           Only run tests matching given mark expression. For example: -m 'mark1 and
                            not mark2'.
      --markers             show markers (builtin, plugin and per-project ones).
      -x, --exitfirst       Exit instantly on first error or failed test
      --fixtures, --funcargs
                            Show available fixtures, sorted by plugin appearance (fixtures with leading
                            '_' are only shown with '-v')
      --fixtures-per-test   Show fixtures per test
      --pdb                 Start the interactive Python debugger on errors or KeyboardInterrupt
      --pdbcls=modulename:classname
                            Specify a custom interactive Python debugger for use with --pdb.For example:
                            --pdbcls=IPython.terminal.debugger:TerminalPdb
      --trace               Immediately break when running each test
      --capture=method      Per-test capturing method: one of fd|sys|no|tee-sys
      -s                    Shortcut for --capture=no
      --runxfail            Report the results of xfail tests as if they were not marked
      --lf, --last-failed   Rerun only the tests that failed at the last run (or all if none failed)
      --ff, --failed-first  Run all tests, but run the last failures first. This may re-order tests and
                            thus lead to repeated fixture setup/teardown.
      --nf, --new-first     Run tests from new files first, then the rest of the tests sorted by file
                            mtime
      --cache-show=[CACHESHOW]
                            Show cache contents, don't perform collection or tests. Optional argument:
                            glob (default: '*').
      --cache-clear         Remove all cache contents at start of test run
      --lfnf={all,none}, --last-failed-no-failures={all,none}
                            With ``--lf``, determines whether to execute tests when there are no
                            previously (known) failures or when no cached ``lastfailed`` data was found.
                            ``all`` (the default) runs the full test suite again. ``none`` just emits a
                            message about no known failures and exits successfully.
      --sw, --stepwise      Exit on test failure and continue from last failing test next time
      --sw-skip, --stepwise-skip
                            Ignore the first failing test but stop on the next failing test. Implicitly
                            enables --stepwise.

    Reporting:
      --durations=N         Show N slowest setup/test durations (N=0 for all)
      --durations-min=N     Minimal duration in seconds for inclusion in slowest list. Default: 0.005.
      -v, --verbose         Increase verbosity
      --no-header           Disable header
      --no-summary          Disable summary
      -q, --quiet           Decrease verbosity
      --verbosity=VERBOSE   Set verbosity. Default: 0.
      -r chars              Show extra test summary info as specified by chars: (f)ailed, (E)rror,
                            (s)kipped, (x)failed, (X)passed, (p)assed, (P)assed with output, (a)ll
                            except passed (p/P), or (A)ll. (w)arnings are enabled by default (see
                            --disable-warnings), 'N' can be used to reset the list. (default: 'fE').
      --disable-warnings, --disable-pytest-warnings
                            Disable warnings summary
      -l, --showlocals      Show locals in tracebacks (disabled by default)
      --no-showlocals       Hide locals in tracebacks (negate --showlocals passed through addopts)
      --tb=style            Traceback print mode (auto/long/short/line/native/no)
      --show-capture={no,stdout,stderr,log,all}
                            Controls how captured stdout/stderr/log is shown on failed tests. Default:
                            all.
      --full-trace          Don't cut any tracebacks (default is to cut)
      --color=color         Color terminal output (yes/no/auto)
      --code-highlight={yes,no}
                            Whether code should be highlighted (only if --color is also enabled).
                            Default: yes.
      --pastebin=mode       Send failed|all info to bpaste.net pastebin service
      --junit-xml=path      Create junit-xml style report file at given path
      --junit-prefix=str    Prepend prefix to classnames in junit-xml output

    pytest-warnings:
      -W PYTHONWARNINGS, --pythonwarnings=PYTHONWARNINGS
                            Set which warnings to report, see -W option of Python itself
      --maxfail=num         Exit after first num failures or errors
      --strict-config       Any warnings encountered while parsing the `pytest` section of the
                            configuration file raise errors
      --strict-markers      Markers not registered in the `markers` section of the configuration file
                            raise errors
      --strict              (Deprecated) alias to --strict-markers
      -c FILE, --config-file=FILE
                            Load configuration from `FILE` instead of trying to locate one of the
                            implicit configuration files.
      --continue-on-collection-errors
                            Force test execution even if collection errors occur
      --rootdir=ROOTDIR     Define root directory for tests. Can be relative path: 'root_dir',
                            './root_dir', 'root_dir/another_dir/'; absolute path: '/home/user/root_dir';
                            path with variables: '$HOME/root_dir'.

    collection:
      --collect-only, --co  Only collect tests, don't execute them
      --pyargs              Try to interpret all arguments as Python packages
      --ignore=path         Ignore path during collection (multi-allowed)
      --ignore-glob=path    Ignore path pattern during collection (multi-allowed)
      --deselect=nodeid_prefix
                            Deselect item (via node id prefix) during collection (multi-allowed)
      --confcutdir=dir      Only load conftest.py's relative to specified dir
      --noconftest          Don't load any conftest.py files
      --keep-duplicates     Keep duplicate tests
      --collect-in-virtualenv
                            Don't ignore tests in a local virtualenv directory
      --import-mode={prepend,append,importlib}
                            Prepend/append to sys.path when importing test modules and conftest files.
                            Default: prepend.
      --doctest-modules     Run doctests in all .py modules
      --doctest-report={none,cdiff,ndiff,udiff,only_first_failure}
                            Choose another output format for diffs on doctest failure
      --doctest-glob=pat    Doctests file matching pattern, default: test*.txt
      --doctest-ignore-import-errors
                            Ignore doctest collection errors
      --doctest-continue-on-failure
                            For a given doctest, continue to run after the first failure

    test session debugging and configuration:
      --basetemp=dir        Base temporary directory for this test run. (Warning: this directory is
                            removed if it exists.)
      -V, --version         Display pytest version and information about plugins. When given twice, also
                            display information about plugins.
      -h, --help            Show help message and configuration info
      -p name               Early-load given plugin module name or entry point (multi-allowed). To avoid
                            loading of plugins, use the `no:` prefix, e.g. `no:doctest`.
      --trace-config        Trace considerations of conftest.py files
      --debug=[DEBUG_FILE_NAME]
                            Store internal tracing debug information in this log file. This file is
                            opened with 'w' and truncated as a result, care advised. Default:
                            pytestdebug.log.
      -o OVERRIDE_INI, --override-ini=OVERRIDE_INI
                            Override ini option with "option=value" style, e.g. `-o xfail_strict=True -o
                            cache_dir=cache`.
      --assert=MODE         Control assertion debugging tools.
                            'plain' performs no assertion debugging.
                            'rewrite' (the default) rewrites assert statements in test modules on import
                            to provide assert expression information.
      --setup-only          Only setup fixtures, do not execute tests
      --setup-show          Show setup of fixtures while executing tests
      --setup-plan          Show what fixtures and tests would be executed but don't execute anything

    logging:
      --log-level=LEVEL     Level of messages to catch/display. Not set by default, so it depends on the
                            root/parent log handler's effective level, where it is "WARNING" by default.
      --log-format=LOG_FORMAT
                            Log format used by the logging module
      --log-date-format=LOG_DATE_FORMAT
                            Log date format used by the logging module
      --log-cli-level=LOG_CLI_LEVEL
                            CLI logging level
      --log-cli-format=LOG_CLI_FORMAT
                            Log format used by the logging module
      --log-cli-date-format=LOG_CLI_DATE_FORMAT
                            Log date format used by the logging module
      --log-file=LOG_FILE   Path to a file when logging will be written to
      --log-file-mode={w,a}
                            Log file open mode
      --log-file-level=LOG_FILE_LEVEL
                            Log file logging level
      --log-file-format=LOG_FILE_FORMAT
                            Log format used by the logging module
      --log-file-date-format=LOG_FILE_DATE_FORMAT
                            Log date format used by the logging module
      --log-auto-indent=LOG_AUTO_INDENT
                            Auto-indent multiline messages passed to the logging module. Accepts
                            true|on, false|off or an integer.
      --log-disable=LOGGER_DISABLE
                            Disable a logger by name. Can be passed multiple times.

    [pytest] ini-options in the first pytest.ini|tox.ini|setup.cfg|pyproject.toml file found:

      markers (linelist):   Register new markers for test functions
      empty_parameter_set_mark (string):
                            Default marker for empty parametersets
      norecursedirs (args): Directory patterns to avoid for recursion
      testpaths (args):     Directories to search for tests when no files or directories are given on
                            the command line
      filterwarnings (linelist):
                            Each line specifies a pattern for warnings.filterwarnings. Processed after
                            -W/--pythonwarnings.
      consider_namespace_packages (bool):
                            Consider namespace packages when resolving module names during import
      usefixtures (args):   List of default fixtures to be used with this project
      python_files (args):  Glob-style file patterns for Python test module discovery
      python_classes (args):
                            Prefixes or glob names for Python test class discovery
      python_functions (args):
                            Prefixes or glob names for Python test function and method discovery
      disable_test_id_escaping_and_forfeit_all_rights_to_community_support (bool):
                            Disable string escape non-ASCII characters, might cause unwanted side
                            effects(use at your own risk)
      console_output_style (string):
                            Console output: "classic", or with additional progress information
                            ("progress" (percentage) | "count" | "progress-even-when-capture-no" (forces
                            progress even when capture=no)
      verbosity_test_cases (string):
                            Specify a verbosity level for test case execution, overriding the main
                            level. Higher levels will provide more detailed information about each test
                            case executed.
      xfail_strict (bool):  Default for the strict parameter of xfail markers when not given explicitly
                            (default: False)
      tmp_path_retention_count (string):
                            How many sessions should we keep the `tmp_path` directories, according to
                            `tmp_path_retention_policy`.
      tmp_path_retention_policy (string):
                            Controls which directories created by the `tmp_path` fixture are kept
                            around, based on test outcome. (all/failed/none)
      enable_assertion_pass_hook (bool):
                            Enables the pytest_assertion_pass hook. Make sure to delete any previously
                            generated pyc cache files.
      verbosity_assertions (string):
                            Specify a verbosity level for assertions, overriding the main level. Higher
                            levels will provide more detailed explanation when an assertion fails.
      junit_suite_name (string):
                            Test suite name for JUnit report
      junit_logging (string):
                            Write captured log messages to JUnit report: one of
                            no|log|system-out|system-err|out-err|all
      junit_log_passing_tests (bool):
                            Capture log information for passing tests to JUnit report:
      junit_duration_report (string):
                            Duration time to report: one of total|call
      junit_family (string):
                            Emit XML for schema: one of legacy|xunit1|xunit2
      doctest_optionflags (args):
                            Option flags for doctests
      doctest_encoding (string):
                            Encoding used for doctest files
      cache_dir (string):   Cache directory path
      log_level (string):   Default value for --log-level
      log_format (string):  Default value for --log-format
      log_date_format (string):
                            Default value for --log-date-format
      log_cli (bool):       Enable log display during test run (also known as "live logging")
      log_cli_level (string):
                            Default value for --log-cli-level
      log_cli_format (string):
                            Default value for --log-cli-format
      log_cli_date_format (string):
                            Default value for --log-cli-date-format
      log_file (string):    Default value for --log-file
      log_file_mode (string):
                            Default value for --log-file-mode
      log_file_level (string):
                            Default value for --log-file-level
      log_file_format (string):
                            Default value for --log-file-format
      log_file_date_format (string):
                            Default value for --log-file-date-format
      log_auto_indent (string):
                            Default value for --log-auto-indent
      pythonpath (paths):   Add paths to sys.path
      faulthandler_timeout (string):
                            Dump the traceback of all threads if a test takes more than TIMEOUT seconds
                            to finish
      addopts (args):       Extra command line options
      minversion (string):  Minimally required pytest version
      required_plugins (args):
                            Plugins that must be present for pytest to run

    Environment variables:
      PYTEST_ADDOPTS           Extra command line options
      PYTEST_PLUGINS           Comma-separated plugins to load during startup
      PYTEST_DISABLE_PLUGIN_AUTOLOAD Set to disable plugin auto-loading
      PYTEST_DEBUG             Set to enable debug tracing of pytest's internals


    to see available markers type: pytest --markers
    to see available fixtures type: pytest --fixtures
    (shown according to specified file_or_dir or current dir if not specified; fixtures with leading '_' are only shown with the '-v' option

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + iniconfig==2.0.0
     + packaging==24.0
     + pluggy==1.4.0
     + pytest==8.1.1
    "#);

    // Can use `--` to separate uv arguments from the command arguments.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--")
        .arg("pytest")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pytest 8.1.1

    ----- stderr -----
    Resolved [N] packages in [TIME]
    "###);
}

#[test]
fn tool_run_at_version() {
    let context = TestContext::new("3.12").with_filtered_exe_suffix();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("pytest@8.0.0")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pytest 8.0.0

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 4 packages in [TIME]
    Installed 4 packages in [TIME]
     + iniconfig==2.0.0
     + packaging==24.0
     + pluggy==1.4.0
     + pytest==8.0.0
    "###);

    // Empty versions are just treated as package and command names
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("pytest@")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Failed to parse: `pytest@`
      Caused by: Expected URL
    pytest@
           ^
    "###);

    // Invalid versions are just treated as package and command names
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("pytest@invalid")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
      × Failed to resolve tool requirement
      ╰─▶ Distribution not found at: file://[TEMP_DIR]/invalid
    "###);

    let filters = context
        .filters()
        .into_iter()
        .chain([(
            // The error message is different on Windows
            "Caused by: program not found",
            "Caused by: No such file or directory (os error 2)",
        )])
        .collect::<Vec<_>>();

    // When `--from` is used, `@` is not treated as a version request
    uv_snapshot!(filters, context.tool_run()
        .arg("--from")
        .arg("pytest")
        .arg("pytest@8.0.0")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: false
    exit_code: 1
    ----- stdout -----
    The executable `pytest@8.0.0` was not found.
    The following executables are provided by `pytest`:
    - py.test
    - pytest
    Consider using `uv tool run --from pytest <EXECUTABLE_NAME>` instead.

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 4 packages in [TIME]
     + iniconfig==2.0.0
     + packaging==24.0
     + pluggy==1.4.0
     + pytest==8.1.1
    warning: An executable named `pytest@8.0.0` is not provided by package `pytest`.
    "###);
}

#[test]
fn tool_run_from_version() {
    let context = TestContext::new("3.12");
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("pytest==8.0.0")
        .arg("pytest")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pytest 8.0.0

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 4 packages in [TIME]
    Installed 4 packages in [TIME]
     + iniconfig==2.0.0
     + packaging==24.0
     + pluggy==1.4.0
     + pytest==8.0.0
    "###);
}

#[test]
fn tool_run_constraints() {
    let context = TestContext::new("3.12");
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    let constraints_txt = context.temp_dir.child("constraints.txt");
    constraints_txt.write_str("pluggy<1.4.0").unwrap();

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--constraints")
        .arg("constraints.txt")
        .arg("pytest")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pytest 8.0.2

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 4 packages in [TIME]
    Installed 4 packages in [TIME]
     + iniconfig==2.0.0
     + packaging==24.0
     + pluggy==1.3.0
     + pytest==8.0.2
    "###);
}

#[test]
fn tool_run_overrides() {
    let context = TestContext::new("3.12");
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    let overrides_txt = context.temp_dir.child("overrides.txt");
    overrides_txt.write_str("pluggy<1.4.0").unwrap();

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--overrides")
        .arg("overrides.txt")
        .arg("pytest")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pytest 8.1.1

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 4 packages in [TIME]
    Installed 4 packages in [TIME]
     + iniconfig==2.0.0
     + packaging==24.0
     + pluggy==1.3.0
     + pytest==8.1.1
    "###);
}

#[test]
fn tool_run_suggest_valid_commands() {
    let context = TestContext::new("3.12").with_filtered_exe_suffix();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    uv_snapshot!(context.filters(), context.tool_run()
    .arg("--from")
    .arg("black")
    .arg("orange")
    .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
    .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: false
    exit_code: 1
    ----- stdout -----
    The executable `orange` was not found.
    The following executables are provided by `black`:
    - black
    - blackd
    Consider using `uv tool run --from black <EXECUTABLE_NAME>` instead.

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Prepared 6 packages in [TIME]
    Installed 6 packages in [TIME]
     + black==24.3.0
     + click==8.1.7
     + mypy-extensions==1.0.0
     + packaging==24.0
     + pathspec==0.12.1
     + platformdirs==4.2.0
    warning: An executable named `orange` is not provided by package `black`.
    "###);

    uv_snapshot!(context.filters(), context.tool_run()
    .arg("fastapi-cli")
    .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
    .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: false
    exit_code: 1
    ----- stdout -----
    The executable `fastapi-cli` was not found.

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + fastapi-cli==0.0.1
     + importlib-metadata==1.7.0
     + zipp==3.18.1
    warning: Package `fastapi-cli` does not provide any executables.
    "###);
}

#[test]
fn tool_run_warn_executable_not_in_from() {
    // FastAPI 0.111 is only available from this date onwards.
    let context = TestContext::new("3.12")
        .with_exclude_newer("2024-05-04T00:00:00Z")
        .with_filtered_exe_suffix();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    let mut filters = context.filters();
    filters.push(("\\+ uvloop(.+)\n ", ""));
    // Strip off the `fastapi` command output.
    filters.push(("(?s)fastapi` instead.*", "fastapi` instead."));

    uv_snapshot!(filters, context.tool_run()
        .arg("--from")
        .arg("fastapi")
        .arg("fastapi")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 35 packages in [TIME]
    Prepared 35 packages in [TIME]
    Installed 35 packages in [TIME]
     + annotated-types==0.6.0
     + anyio==4.3.0
     + certifi==2024.2.2
     + click==8.1.7
     + dnspython==2.6.1
     + email-validator==2.1.1
     + fastapi==0.111.0
     + fastapi-cli==0.0.2
     + h11==0.14.0
     + httpcore==1.0.5
     + httptools==0.6.1
     + httpx==0.27.0
     + idna==3.7
     + jinja2==3.1.3
     + markdown-it-py==3.0.0
     + markupsafe==2.1.5
     + mdurl==0.1.2
     + orjson==3.10.3
     + pydantic==2.7.1
     + pydantic-core==2.18.2
     + pygments==2.17.2
     + python-dotenv==1.0.1
     + python-multipart==0.0.9
     + pyyaml==6.0.1
     + rich==13.7.1
     + shellingham==1.5.4
     + sniffio==1.3.1
     + starlette==0.37.2
     + typer==0.12.3
     + typing-extensions==4.11.0
     + ujson==5.9.0
     + uvicorn==0.29.0
     + watchfiles==0.21.0
     + websockets==12.0
    warning: An executable named `fastapi` is not provided by package `fastapi` but is available via the dependency `fastapi-cli`. Consider using `uv tool run --from fastapi-cli fastapi` instead.
    "###);
}

#[test]
fn tool_run_from_install() {
    let context = TestContext::new("3.12").with_filtered_counts();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    // Install `black` at a specific version.
    context
        .tool_install()
        .arg("black==24.1.0")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str())
        .assert()
        .success();

    // Verify that `tool run black` uses the already-installed version.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("black")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    black, 24.1.0 (compiled: yes)
    Python (CPython) 3.12.[X]

    ----- stderr -----
    "###);

    // Verify that `--isolated` uses an isolated environment.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--isolated")
        .arg("black")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    black, 24.3.0 (compiled: yes)
    Python (CPython) 3.12.[X]

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + black==24.3.0
     + click==8.1.7
     + mypy-extensions==1.0.0
     + packaging==24.0
     + pathspec==0.12.1
     + platformdirs==4.2.0
    "###);

    // Verify that `tool run black` at a different version installs the new version.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("black@24.1.1")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    black, 24.1.1 (compiled: yes)
    Python (CPython) 3.12.[X]

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + black==24.1.1
     + click==8.1.7
     + mypy-extensions==1.0.0
     + packaging==24.0
     + pathspec==0.12.1
     + platformdirs==4.2.0
    "###);

    // Verify that `--with` installs a new version.
    // TODO(charlie): This could (in theory) layer the `--with` requirements on top of the existing
    // environment.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--with")
        .arg("iniconfig")
        .arg("black")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    black, 24.3.0 (compiled: yes)
    Python (CPython) 3.12.[X]

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + black==24.3.0
     + click==8.1.7
     + iniconfig==2.0.0
     + mypy-extensions==1.0.0
     + packaging==24.0
     + pathspec==0.12.1
     + platformdirs==4.2.0
    "###);

    // Verify that `tool run black` at a different version (via `--from`) installs the new version.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("black==24.2.0")
        .arg("black")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    black, 24.2.0 (compiled: yes)
    Python (CPython) 3.12.[X]

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + black==24.2.0
     + click==8.1.7
     + mypy-extensions==1.0.0
     + packaging==24.0
     + pathspec==0.12.1
     + platformdirs==4.2.0
    "###);
}

#[test]
fn tool_run_from_install_constraints() {
    let context = TestContext::new("3.12").with_filtered_counts();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    // Install `flask` at a specific version.
    context
        .tool_install()
        .arg("flask==3.0.0")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str())
        .assert()
        .success();

    // Verify that `tool run flask` uses the already-installed version.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("flask")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 3.0.0
    Werkzeug 3.0.1

    ----- stderr -----
    "###);

    // Verify that `tool run flask` with a compatible constraint uses the already-installed version.
    context
        .temp_dir
        .child("constraints.txt")
        .write_str("werkzeug<4.0.0")
        .unwrap();

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--constraints")
        .arg("constraints.txt")
        .arg("flask")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 3.0.0
    Werkzeug 3.0.1

    ----- stderr -----
    "###);

    // Verify that `tool run flask` with an incompatible constraint installs a new version.
    context
        .temp_dir
        .child("constraints.txt")
        .write_str("werkzeug<3.0.0")
        .unwrap();

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--constraints")
        .arg("constraints.txt")
        .arg("flask")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 2.3.3
    Werkzeug 2.3.8

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + blinker==1.7.0
     + click==8.1.7
     + flask==2.3.3
     + itsdangerous==2.1.2
     + jinja2==3.1.3
     + markupsafe==2.1.5
     + werkzeug==2.3.8
    "###);

    // Verify that `tool run flask` with a compatible override uses the already-installed version.
    context
        .temp_dir
        .child("override.txt")
        .write_str("werkzeug==3.0.1")
        .unwrap();

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--override")
        .arg("override.txt")
        .arg("flask")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 3.0.0
    Werkzeug 3.0.1

    ----- stderr -----
    "###);

    // Verify that `tool run flask` with an incompatible override installs a new version.
    context
        .temp_dir
        .child("override.txt")
        .write_str("werkzeug==3.0.0")
        .unwrap();

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--override")
        .arg("override.txt")
        .arg("flask")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 3.0.2
    Werkzeug 3.0.0

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + blinker==1.7.0
     + click==8.1.7
     + flask==3.0.2
     + itsdangerous==2.1.2
     + jinja2==3.1.3
     + markupsafe==2.1.5
     + werkzeug==3.0.0
    "###);

    // Verify that an override that enables a new extra also invalidates the environment.
    context
        .temp_dir
        .child("override.txt")
        .write_str("flask[dotenv]")
        .unwrap();

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--override")
        .arg("override.txt")
        .arg("flask")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 3.0.2
    Werkzeug 3.0.1

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + blinker==1.7.0
     + click==8.1.7
     + flask==3.0.2
     + itsdangerous==2.1.2
     + jinja2==3.1.3
     + markupsafe==2.1.5
     + python-dotenv==1.0.1
     + werkzeug==3.0.1
    "###);
}

#[test]
fn tool_run_cache() {
    let context = TestContext::new_with_versions(&["3.11", "3.12"]).with_filtered_counts();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    // Verify that `tool run black` installs the latest version.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("-p")
        .arg("3.12")
        .arg("black")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    black, 24.3.0 (compiled: yes)
    Python (CPython) 3.12.[X]

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + black==24.3.0
     + click==8.1.7
     + mypy-extensions==1.0.0
     + packaging==24.0
     + pathspec==0.12.1
     + platformdirs==4.2.0
    "###);

    // Verify that `tool run black` uses the cached version.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("-p")
        .arg("3.12")
        .arg("black")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    black, 24.3.0 (compiled: yes)
    Python (CPython) 3.12.[X]

    ----- stderr -----
    Resolved [N] packages in [TIME]
    "###);

    // Verify that `--refresh` recreates everything.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("-p")
        .arg("3.12")
        .arg("--refresh")
        .arg("black")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    black, 24.3.0 (compiled: yes)
    Python (CPython) 3.12.[X]

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + black==24.3.0
     + click==8.1.7
     + mypy-extensions==1.0.0
     + packaging==24.0
     + pathspec==0.12.1
     + platformdirs==4.2.0
    "###);

    // Verify that `--refresh-package` recreates everything. We may want to change this.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("-p")
        .arg("3.12")
        .arg("--refresh-package")
        .arg("packaging")
        .arg("black")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    black, 24.3.0 (compiled: yes)
    Python (CPython) 3.12.[X]

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + black==24.3.0
     + click==8.1.7
     + mypy-extensions==1.0.0
     + packaging==24.0
     + pathspec==0.12.1
     + platformdirs==4.2.0
    "###);

    // Verify that varying the interpreter leads to a fresh environment.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("-p")
        .arg("3.11")
        .arg("black")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    black, 24.3.0 (compiled: yes)
    Python (CPython) 3.11.[X]

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + black==24.3.0
     + click==8.1.7
     + mypy-extensions==1.0.0
     + packaging==24.0
     + pathspec==0.12.1
     + platformdirs==4.2.0
    "###);

    // But that re-invoking with the previous interpreter retains the cached version.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("-p")
        .arg("3.12")
        .arg("black")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    black, 24.3.0 (compiled: yes)
    Python (CPython) 3.12.[X]

    ----- stderr -----
    Resolved [N] packages in [TIME]
    "###);

    // Verify that `--with` leads to a fresh environment.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("-p")
        .arg("3.12")
        .arg("--with")
        .arg("iniconfig")
        .arg("black")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    black, 24.3.0 (compiled: yes)
    Python (CPython) 3.12.[X]

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + black==24.3.0
     + click==8.1.7
     + iniconfig==2.0.0
     + mypy-extensions==1.0.0
     + packaging==24.0
     + pathspec==0.12.1
     + platformdirs==4.2.0
    "###);
}

#[test]
fn tool_run_url() {
    let context = TestContext::new("3.12").with_filtered_counts();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("flask @ https://files.pythonhosted.org/packages/61/80/ffe1da13ad9300f87c93af113edd0638c75138c42a0994becfacac078c06/flask-3.0.3-py3-none-any.whl")
        .arg("flask")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 3.0.3
    Werkzeug 3.0.1

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + blinker==1.7.0
     + click==8.1.7
     + flask==3.0.3 (from https://files.pythonhosted.org/packages/61/80/ffe1da13ad9300f87c93af113edd0638c75138c42a0994becfacac078c06/flask-3.0.3-py3-none-any.whl)
     + itsdangerous==2.1.2
     + jinja2==3.1.3
     + markupsafe==2.1.5
     + werkzeug==3.0.1
    "###);

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("https://files.pythonhosted.org/packages/61/80/ffe1da13ad9300f87c93af113edd0638c75138c42a0994becfacac078c06/flask-3.0.3-py3-none-any.whl")
        .arg("flask")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 3.0.3
    Werkzeug 3.0.1

    ----- stderr -----
    Resolved [N] packages in [TIME]
    "###);

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("flask @ https://files.pythonhosted.org/packages/61/80/ffe1da13ad9300f87c93af113edd0638c75138c42a0994becfacac078c06/flask-3.0.3-py3-none-any.whl")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 3.0.3
    Werkzeug 3.0.1

    ----- stderr -----
    Resolved [N] packages in [TIME]
    "###);

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("https://files.pythonhosted.org/packages/61/80/ffe1da13ad9300f87c93af113edd0638c75138c42a0994becfacac078c06/flask-3.0.3-py3-none-any.whl")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 3.0.3
    Werkzeug 3.0.1

    ----- stderr -----
    Resolved [N] packages in [TIME]
    "###);
}

/// Read requirements from a `requirements.txt` file.
#[test]
fn tool_run_requirements_txt() {
    let context = TestContext::new("3.12").with_filtered_counts();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    let requirements_txt = context.temp_dir.child("requirements.txt");
    requirements_txt.write_str("iniconfig").unwrap();

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--with-requirements")
        .arg("requirements.txt")
        .arg("--with")
        .arg("typing-extensions")
        .arg("flask")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 3.0.2
    Werkzeug 3.0.1

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + blinker==1.7.0
     + click==8.1.7
     + flask==3.0.2
     + iniconfig==2.0.0
     + itsdangerous==2.1.2
     + jinja2==3.1.3
     + markupsafe==2.1.5
     + typing-extensions==4.10.0
     + werkzeug==3.0.1
    "###);
}

/// Ignore and warn when (e.g.) the `--index-url` argument is a provided `requirements.txt`.
#[test]
fn tool_run_requirements_txt_arguments() {
    let context = TestContext::new("3.12").with_filtered_counts();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    let requirements_txt = context.temp_dir.child("requirements.txt");
    requirements_txt
        .write_str(indoc! { r"
        --index-url https://test.pypi.org/simple
        idna
        "
        })
        .unwrap();

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--with-requirements")
        .arg("requirements.txt")
        .arg("flask")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 3.0.2
    Werkzeug 3.0.1

    ----- stderr -----
    warning: Ignoring `--index-url` from requirements file: `https://test.pypi.org/simple`. Instead, use the `--index-url` command-line argument, or set `index-url` in a `uv.toml` or `pyproject.toml` file.
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + blinker==1.7.0
     + click==8.1.7
     + flask==3.0.2
     + idna==3.6
     + itsdangerous==2.1.2
     + jinja2==3.1.3
     + markupsafe==2.1.5
     + werkzeug==3.0.1
    "###);
}

/// List installed tools when no command arg is given (e.g. `uv tool run`).
#[test]
fn tool_run_list_installed() {
    let context = TestContext::new("3.12").with_filtered_exe_suffix();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    // No tools installed.
    uv_snapshot!(context.filters(), context.tool_run()
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: false
    exit_code: 2
    ----- stdout -----
    Provide a command to run with `uv tool run <command>`.

    See `uv tool run --help` for more information.

    ----- stderr -----
    "###);

    // Install `black`.
    context
        .tool_install()
        .arg("black==24.2.0")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str())
        .assert()
        .success();

    // List installed tools.
    uv_snapshot!(context.filters(), context.tool_run()
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: false
    exit_code: 2
    ----- stdout -----
    Provide a command to run with `uv tool run <command>`.

    The following tools are installed:

    - black v24.2.0

    See `uv tool run --help` for more information.

    ----- stderr -----
    "###);
}

/// By default, omit resolver and installer output.
#[test]
fn tool_run_without_output() {
    let context = TestContext::new("3.12").with_filtered_counts();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    // On the first run, only show the summary line.
    uv_snapshot!(context.filters(), context.tool_run()
        .env_remove(EnvVars::UV_SHOW_RESOLUTION)
        .arg("--")
        .arg("pytest")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pytest 8.1.1

    ----- stderr -----
    Installed [N] packages in [TIME]
    "###);

    // Subsequent runs are quiet.
    uv_snapshot!(context.filters(), context.tool_run()
        .env_remove(EnvVars::UV_SHOW_RESOLUTION)
        .arg("--")
        .arg("pytest")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pytest 8.1.1

    ----- stderr -----
    "###);
}

#[test]
#[cfg(not(windows))]
fn tool_run_csv_with() -> anyhow::Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    let anyio_local = context.temp_dir.child("src").child("anyio_local");
    copy_dir_all(
        context.workspace_root.join("scripts/packages/anyio_local"),
        &anyio_local,
    )?;

    let black_editable = context.temp_dir.child("src").child("black_editable");
    copy_dir_all(
        context
            .workspace_root
            .join("scripts/packages/black_editable"),
        &black_editable,
    )?;

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(indoc! { r#"
        [project]
        name = "foo"
        version = "1.0.0"
        requires-python = ">=3.8"
        dependencies = ["anyio", "sniffio==1.3.1"]
        "#
    })?;

    let test_script = context.temp_dir.child("main.py");
    test_script.write_str(indoc! { r"
        import sniffio
       "
    })?;

    // Performs a tool run with a comma-separated `--with` flag.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--with")
        .arg("iniconfig,typing-extensions")
        .arg("pytest")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pytest 8.1.1

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + iniconfig==2.0.0
     + packaging==24.0
     + pluggy==1.4.0
     + pytest==8.1.1
     + typing-extensions==4.10.0
    "###);

    Ok(())
}

#[test]
#[cfg(windows)]
fn tool_run_csv_with() -> anyhow::Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    let anyio_local = context.temp_dir.child("src").child("anyio_local");
    copy_dir_all(
        context.workspace_root.join("scripts/packages/anyio_local"),
        &anyio_local,
    )?;

    let black_editable = context.temp_dir.child("src").child("black_editable");
    copy_dir_all(
        context
            .workspace_root
            .join("scripts/packages/black_editable"),
        &black_editable,
    )?;

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(indoc! { r#"
        [project]
        name = "foo"
        version = "1.0.0"
        requires-python = ">=3.8"
        dependencies = ["anyio", "sniffio==1.3.1"]
        "#
    })?;

    let test_script = context.temp_dir.child("main.py");
    test_script.write_str(indoc! { r"
        import sniffio
       "
    })?;

    // Performs a tool run with a comma-separated `--with` flag.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--with")
        .arg("iniconfig,typing-extensions")
        .arg("pytest")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pytest 8.1.1

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + iniconfig==2.0.0
     + packaging==24.0
     + pluggy==1.4.0
     + pytest==8.1.1
     + typing-extensions==4.10.0
    "###);

    Ok(())
}

#[test]
#[cfg(not(windows))]
fn tool_run_repeated_with() -> anyhow::Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    let anyio_local = context.temp_dir.child("src").child("anyio_local");
    copy_dir_all(
        context.workspace_root.join("scripts/packages/anyio_local"),
        &anyio_local,
    )?;

    let black_editable = context.temp_dir.child("src").child("black_editable");
    copy_dir_all(
        context
            .workspace_root
            .join("scripts/packages/black_editable"),
        &black_editable,
    )?;

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(indoc! { r#"
        [project]
        name = "foo"
        version = "1.0.0"
        requires-python = ">=3.8"
        dependencies = ["anyio", "sniffio==1.3.1"]
        "#
    })?;

    let test_script = context.temp_dir.child("main.py");
    test_script.write_str(indoc! { r"
        import sniffio
       "
    })?;

    // Performs a tool run with a repeated `--with` flag.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--with")
        .arg("iniconfig")
        .arg("--with")
        .arg("typing-extensions")
        .arg("pytest")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pytest 8.1.1

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + iniconfig==2.0.0
     + packaging==24.0
     + pluggy==1.4.0
     + pytest==8.1.1
     + typing-extensions==4.10.0
    "###);

    Ok(())
}

#[test]
#[cfg(windows)]
fn tool_run_repeated_with() -> anyhow::Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    let anyio_local = context.temp_dir.child("src").child("anyio_local");
    copy_dir_all(
        context.workspace_root.join("scripts/packages/anyio_local"),
        &anyio_local,
    )?;

    let black_editable = context.temp_dir.child("src").child("black_editable");
    copy_dir_all(
        context
            .workspace_root
            .join("scripts/packages/black_editable"),
        &black_editable,
    )?;

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(indoc! { r#"
        [project]
        name = "foo"
        version = "1.0.0"
        requires-python = ">=3.8"
        dependencies = ["anyio", "sniffio==1.3.1"]
        "#
    })?;

    let test_script = context.temp_dir.child("main.py");
    test_script.write_str(indoc! { r"
        import sniffio
       "
    })?;

    // Performs a tool run with a repeated `--with` flag.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--with")
        .arg("iniconfig")
        .arg("--with")
        .arg("typing-extensions")
        .arg("pytest")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pytest 8.1.1

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + iniconfig==2.0.0
     + packaging==24.0
     + pluggy==1.4.0
     + pytest==8.1.1
     + typing-extensions==4.10.0
    "###);

    Ok(())
}

#[test]
fn tool_run_with_editable() -> anyhow::Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    let anyio_local = context.temp_dir.child("src").child("anyio_local");
    copy_dir_all(
        context.workspace_root.join("scripts/packages/anyio_local"),
        &anyio_local,
    )?;

    let black_editable = context.temp_dir.child("src").child("black_editable");
    copy_dir_all(
        context
            .workspace_root
            .join("scripts/packages/black_editable"),
        &black_editable,
    )?;

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(indoc! { r#"
        [project]
        name = "foo"
        version = "1.0.0"
        requires-python = ">=3.8"
        dependencies = ["anyio", "sniffio==1.3.1"]
        "#
    })?;

    let test_script = context.temp_dir.child("main.py");
    test_script.write_str(indoc! { r"
        import sniffio
       "
    })?;

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--with-editable")
        .arg("./src/black_editable")
        .arg("--with")
        .arg("iniconfig")
        .arg("flask")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 3.0.2
    Werkzeug 3.0.1

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + black==0.1.0 (from file://[TEMP_DIR]/src/black_editable)
     + blinker==1.7.0
     + click==8.1.7
     + flask==3.0.2
     + iniconfig==2.0.0
     + itsdangerous==2.1.2
     + jinja2==3.1.3
     + markupsafe==2.1.5
     + werkzeug==3.0.1
    "###);

    // Requesting an editable requirement should install it in a layer, even if it satisfied
    uv_snapshot!(context.filters(), context.tool_run().arg("--with-editable").arg("./src/anyio_local").arg("flask").arg("--version").env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str()).env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()),
    @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 3.0.2
    Werkzeug 3.0.1

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + anyio==4.3.0+foo (from file://[TEMP_DIR]/src/anyio_local)
     + blinker==1.7.0
     + click==8.1.7
     + flask==3.0.2
     + itsdangerous==2.1.2
     + jinja2==3.1.3
     + markupsafe==2.1.5
     + werkzeug==3.0.1
    "###);

    // Requesting the project itself should use a new environment.
    uv_snapshot!(context.filters(), context.tool_run().arg("--with-editable").arg(".").arg("flask").arg("--version").env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str()).env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 3.0.2
    Werkzeug 3.0.1

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + anyio==4.3.0
     + blinker==1.7.0
     + click==8.1.7
     + flask==3.0.2
     + foo==1.0.0 (from file://[TEMP_DIR]/)
     + idna==3.6
     + itsdangerous==2.1.2
     + jinja2==3.1.3
     + markupsafe==2.1.5
     + sniffio==1.3.1
     + werkzeug==3.0.1
    "###);

    // If invalid, we should reference `--with`.
    uv_snapshot!(context.filters(), context
        .tool_run()
        .arg("--with")
        .arg("./foo")
        .arg("flask")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir
        .as_os_str()).env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
      × Failed to resolve `--with` requirement
      ╰─▶ Distribution not found at: file://[TEMP_DIR]/foo
    "###);

    Ok(())
}

#[test]
fn warn_no_executables_found() {
    let context = TestContext::new("3.12").with_filtered_exe_suffix();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("requests")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: false
    exit_code: 1
    ----- stdout -----
    The executable `requests` was not found.

    ----- stderr -----
    Resolved 5 packages in [TIME]
    Prepared 5 packages in [TIME]
    Installed 5 packages in [TIME]
     + certifi==2024.2.2
     + charset-normalizer==3.3.2
     + idna==3.6
     + requests==2.31.0
     + urllib3==2.2.1
    warning: Package `requests` does not provide any executables.
    "###);
}

/// Warn when a user passes `--upgrade` to `uv tool run`.
#[test]
fn tool_run_upgrade_warn() {
    let context = TestContext::new("3.12").with_filtered_counts();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--upgrade")
        .arg("pytest")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pytest 8.1.1

    ----- stderr -----
    warning: Tools cannot be upgraded via `uv tool run`; use `uv tool upgrade --all` to upgrade all installed tools, or `uv tool run package@latest` to run the latest version of a tool.
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + iniconfig==2.0.0
     + packaging==24.0
     + pluggy==1.4.0
     + pytest==8.1.1
    "###);

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--upgrade")
        .arg("--with")
        .arg("typing-extensions")
        .arg("pytest")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pytest 8.1.1

    ----- stderr -----
    warning: Tools cannot be upgraded via `uv tool run`; use `uv tool upgrade --all` to upgrade all installed tools, `uv tool run package@latest` to run the latest version of a tool, or `uv tool run --refresh package` to upgrade any `--with` dependencies.
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + iniconfig==2.0.0
     + packaging==24.0
     + pluggy==1.4.0
     + pytest==8.1.1
     + typing-extensions==4.10.0
    "###);
}

/// If we fail to resolve the tool, we should include "tool" in the error message.
#[test]
fn tool_run_resolution_error() {
    let context = TestContext::new("3.12").with_filtered_counts();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("add")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
      × No solution found when resolving tool dependencies:
      ╰─▶ Because there are no versions of add and you require add, we can conclude that your requirements are unsatisfiable.
    "###);
}

#[test]
fn tool_run_latest() {
    let context = TestContext::new("3.12").with_filtered_exe_suffix();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    // Install `pytest` at a specific version.
    context
        .tool_install()
        .arg("pytest==7.0.0")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str())
        .assert()
        .success();

    // Run `pytest`, which should use the installed version.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("pytest")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pytest 7.0.0

    ----- stderr -----
    "###);

    // Run `pytest@latest`, which should use the latest version.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("pytest@latest")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pytest 8.1.1

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 4 packages in [TIME]
    Installed 4 packages in [TIME]
     + iniconfig==2.0.0
     + packaging==24.0
     + pluggy==1.4.0
     + pytest==8.1.1
    "###);

    // Run `pytest`, which should use the installed version.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("pytest")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pytest 7.0.0

    ----- stderr -----
    "###);
}

#[test]
fn tool_run_latest_extra() {
    let context = TestContext::new("3.12").with_filtered_exe_suffix();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("flask[dotenv]@latest")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 3.0.2
    Werkzeug 3.0.1

    ----- stderr -----
    Resolved 8 packages in [TIME]
    Prepared 8 packages in [TIME]
    Installed 8 packages in [TIME]
     + blinker==1.7.0
     + click==8.1.7
     + flask==3.0.2
     + itsdangerous==2.1.2
     + jinja2==3.1.3
     + markupsafe==2.1.5
     + python-dotenv==1.0.1
     + werkzeug==3.0.1
    "###);

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("flask[dotenv]@3.0.0")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 3.0.0
    Werkzeug 3.0.1

    ----- stderr -----
    Resolved 8 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 8 packages in [TIME]
     + blinker==1.7.0
     + click==8.1.7
     + flask==3.0.0
     + itsdangerous==2.1.2
     + jinja2==3.1.3
     + markupsafe==2.1.5
     + python-dotenv==1.0.1
     + werkzeug==3.0.1
    "###);
}

#[test]
fn tool_run_extra() {
    let context = TestContext::new("3.12").with_filtered_exe_suffix();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("flask[dotenv]")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 3.0.2
    Werkzeug 3.0.1

    ----- stderr -----
    Resolved 8 packages in [TIME]
    Prepared 8 packages in [TIME]
    Installed 8 packages in [TIME]
     + blinker==1.7.0
     + click==8.1.7
     + flask==3.0.2
     + itsdangerous==2.1.2
     + jinja2==3.1.3
     + markupsafe==2.1.5
     + python-dotenv==1.0.1
     + werkzeug==3.0.1
    "###);
}

#[test]
fn tool_run_specifier() {
    let context = TestContext::new("3.12").with_filtered_exe_suffix();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("flask<3.0.0")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]
    Flask 2.3.3
    Werkzeug 3.0.1

    ----- stderr -----
    Resolved 7 packages in [TIME]
    Prepared 7 packages in [TIME]
    Installed 7 packages in [TIME]
     + blinker==1.7.0
     + click==8.1.7
     + flask==2.3.3
     + itsdangerous==2.1.2
     + jinja2==3.1.3
     + markupsafe==2.1.5
     + werkzeug==3.0.1
    "###);
}

#[test]
fn tool_run_python() {
    let context = TestContext::new("3.12").with_filtered_counts();
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("python")
        .arg("--version"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]

    ----- stderr -----
    Resolved in [TIME]
    Audited in [TIME]
    "###);

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("python")
        .arg("-c")
        .arg("print('Hello, world!')"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Hello, world!

    ----- stderr -----
    Resolved in [TIME]
    "###);
}

#[test]
fn tool_run_python_at_version() {
    let context = TestContext::new_with_versions(&["3.12", "3.11"])
        .with_filtered_counts()
        .with_filtered_python_sources();

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("python")
        .arg("--version"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]

    ----- stderr -----
    Resolved in [TIME]
    Audited in [TIME]
    "###);

    uv_snapshot!(context.filters(), context.tool_run()
            .arg("python@3.12")
            .arg("--version"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]

    ----- stderr -----
    Resolved in [TIME]
    "###);

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("python@3.11")
        .arg("--version"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.11.[X]

    ----- stderr -----
    Resolved in [TIME]
    Audited in [TIME]
    "###);

    // Request a version via `-p`
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("-p")
        .arg("3.11")
        .arg("python")
        .arg("--version"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.11.[X]

    ----- stderr -----
    Resolved in [TIME]
    "###);

    // Request a version in the tool and `-p`
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("-p")
        .arg("3.12")
        .arg("python@3.11")
        .arg("--version"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Received multiple Python version requests: `3.12` and `3.11`
    "###);

    // Request a version that does not exist
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("python@3.12.99"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: No interpreter found for Python 3.12.[X] in [PYTHON SOURCES]
    "###);

    // Request an invalid version
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("python@3.300"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Invalid version request: 3.300
    "###);

    // Request `@latest` (not yet supported)
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("python@latest")
        .arg("--version"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Requesting the 'latest' Python version is not yet supported
    "###);
}

#[test]
fn tool_run_python_from() {
    let context = TestContext::new_with_versions(&["3.12", "3.11"])
        .with_filtered_counts()
        .with_filtered_python_sources();

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("python")
        .arg("python")
        .arg("--version"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.12.[X]

    ----- stderr -----
    Resolved in [TIME]
    Audited in [TIME]
    "###);

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("python@3.11")
        .arg("python")
        .arg("--version"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.11.[X]

    ----- stderr -----
    Resolved in [TIME]
    Audited in [TIME]
    "###);

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("python>=3.12")
        .arg("python")
        .arg("--version"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Using `--from python<specifier>` is not supported. Use `python@<version>` instead.
    "###);
}

#[test]
fn run_with_env_file() -> anyhow::Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    // Create a project with a custom script.
    let foo_dir = context.temp_dir.child("foo");
    let foo_pyproject_toml = foo_dir.child("pyproject.toml");

    foo_pyproject_toml.write_str(indoc! { r#"
        [project]
        name = "foo"
        version = "1.0.0"
        requires-python = ">=3.8"
        dependencies = []

        [project.scripts]
        script = "foo.main:run"

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#
    })?;

    // Create the `foo` module.
    let foo_project_src = foo_dir.child("src");
    let foo_module = foo_project_src.child("foo");
    let foo_main_py = foo_module.child("main.py");
    foo_main_py.write_str(indoc! { r#"
        def run():
            import os

            print(os.environ.get('THE_EMPIRE_VARIABLE'))
            print(os.environ.get('REBEL_1'))
            print(os.environ.get('REBEL_2'))
            print(os.environ.get('REBEL_3'))

        __name__ == "__main__" and run()
       "#
    })?;

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("./foo")
        .arg("script")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    None
    None
    None
    None

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + foo==1.0.0 (from file://[TEMP_DIR]/foo)
    ");

    context.temp_dir.child(".file").write_str(indoc! { "
        THE_EMPIRE_VARIABLE=palpatine
        REBEL_1=leia_organa
        REBEL_2=obi_wan_kenobi
        REBEL_3=C3PO
       "
    })?;

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--env-file").arg(".file")
        .arg("--from")
        .arg("./foo")
        .arg("script")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    palpatine
    leia_organa
    obi_wan_kenobi
    C3PO

    ----- stderr -----
    Resolved [N] packages in [TIME]
    ");

    Ok(())
}

#[test]
fn tool_run_from_at() {
    let context = TestContext::new("3.12")
        .with_exclude_newer("2025-01-18T00:00:00Z")
        .with_filtered_exe_suffix();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("executable-application@latest")
        .arg("app")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    app 0.3.0

    ----- stderr -----
    Resolved 1 package in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + executable-application==0.3.0
    "###);

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("executable-application@0.2.0")
        .arg("app")
        .arg("--version")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    app 0.2.0

    ----- stderr -----
    Resolved 1 package in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + executable-application==0.2.0
    "###);
}

#[test]
fn tool_run_verbatim_name() {
    let context = TestContext::new("3.12")
        .with_filtered_counts()
        .with_filtered_exe_suffix();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    // The normalized package name is `change-wheel-version`, but the executable is `change_wheel_version`.
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("change_wheel_version")
        .arg("--help")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    usage: change_wheel_version [-h] [--local-version LOCAL_VERSION] [--version VERSION]
                                [--delete-old-wheel] [--allow-same-version]
                                wheel

    positional arguments:
      wheel

    options:
      -h, --help            show this help message and exit
      --local-version LOCAL_VERSION
      --version VERSION
      --delete-old-wheel
      --allow-same-version

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + change-wheel-version==0.5.0
     + installer==0.7.0
     + packaging==24.0
     + wheel==0.43.0
    ");

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("change-wheel-version")
        .arg("--help")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: false
    exit_code: 1
    ----- stdout -----
    The executable `change-wheel-version` was not found.
    The following executables are provided by `change-wheel-version`:
    - change_wheel_version
    Consider using `uv tool run --from change-wheel-version <EXECUTABLE_NAME>` instead.

    ----- stderr -----
    Resolved [N] packages in [TIME]
    warning: An executable named `change-wheel-version` is not provided by package `change-wheel-version`.
    "###);

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("change-wheel-version")
        .arg("change_wheel_version")
        .arg("--help")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    usage: change_wheel_version [-h] [--local-version LOCAL_VERSION] [--version VERSION]
                                [--delete-old-wheel] [--allow-same-version]
                                wheel

    positional arguments:
      wheel

    options:
      -h, --help            show this help message and exit
      --local-version LOCAL_VERSION
      --version VERSION
      --delete-old-wheel
      --allow-same-version

    ----- stderr -----
    Resolved [N] packages in [TIME]
    ");
}

#[test]
fn tool_run_with_existing_py_script() -> anyhow::Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();
    context.temp_dir.child("script.py").touch()?;

    uv_snapshot!(context.filters(), context.tool_run().arg("script.py"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: It looks you tried to run a Python script at `script.py`, which is not supported by `uv tool run`

    hint: Use `uv run script.py` instead
    ");
    Ok(())
}

#[test]
fn tool_run_with_existing_pyw_script() -> anyhow::Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();
    context.temp_dir.child("script.pyw").touch()?;

    // We treat arguments before the command as uv arguments
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("script.pyw"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: It looks you tried to run a Python script at `script.pyw`, which is not supported by `uv tool run`

    hint: Use `uv run script.pyw` instead
    ");
    Ok(())
}

#[test]
fn tool_run_with_nonexistent_py_script() {
    let context = TestContext::new("3.12").with_filtered_counts();

    // We treat arguments before the command as uv arguments
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("script.py"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: It looks you provided a Python script to run, which is not supported supported by `uv tool run`

    hint: We did not find a script at the requested path. If you meant to run a command from the `script-py` package, pass the normalized package name to `--from` to disambiguate, e.g., `uv tool run --from script-py script.py`
    ");
}

#[test]
fn tool_run_with_nonexistent_pyw_script() {
    let context = TestContext::new("3.12").with_filtered_counts();

    // We treat arguments before the command as uv arguments
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("script.pyw"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: It looks you provided a Python script to run, which is not supported supported by `uv tool run`

    hint: We did not find a script at the requested path. If you meant to run a command from the `script-pyw` package, pass the normalized package name to `--from` to disambiguate, e.g., `uv tool run --from script-pyw script.pyw`
    ");
}

#[test]
fn tool_run_with_from_script() {
    let context = TestContext::new("3.12").with_filtered_counts();

    // We treat arguments before the command as uv arguments
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("script.py")
        .arg("ruff"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: It looks you provided a Python script to `--from`, which is not supported

    hint: If you meant to run a command from the `script-py` package, use the normalized package name instead to disambiguate, e.g., `uv tool run --from script-py ruff`
    ");
}

#[test]
fn tool_run_with_script_and_from_script() {
    let context = TestContext::new("3.12").with_filtered_counts();

    // We treat arguments before the command as uv arguments
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("script.py")
        .arg("other-script.py"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: It looks you provided a Python script to `--from`, which is not supported

    hint: If you meant to run a command from the `script-py` package, use the normalized package name instead to disambiguate, e.g., `uv tool run --from script-py other-script.py`
    ");
}

/// Test windows runnable types, namely console scripts and legacy setuptools scripts.
/// Console Scripts <https://packaging.python.org/en/latest/guides/writing-pyproject-toml/#console-scripts>
/// Legacy Scripts <https://packaging.python.org/en/latest/guides/distributing-packages-using-setuptools/#scripts>.
///
/// This tests for uv tool run of windows runnable types defined by [`WindowsRunnable`].
#[cfg(windows)]
#[test]
fn tool_run_windows_runnable_types() -> anyhow::Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    let foo_dir = context.temp_dir.child("foo");
    let foo_pyproject_toml = foo_dir.child("pyproject.toml");

    // Use `script-files` which enables legacy scripts packaging.
    foo_pyproject_toml.write_str(indoc! { r#"
        [project]
        name = "foo"
        version = "1.0.0"
        requires-python = ">=3.8"
        dependencies = []

        [project.scripts]
        custom_pydoc = "foo.main:run"

        [tool.setuptools]
        script-files = [
            "misc/custom_pydoc.bat",
            "misc/custom_pydoc.cmd",
            "misc/custom_pydoc.ps1"
        ]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#
    })?;

    // Create the legacy scripts
    let custom_pydoc_bat = foo_dir.child("misc").child("custom_pydoc.bat");
    let custom_pydoc_cmd = foo_dir.child("misc").child("custom_pydoc.cmd");
    let custom_pydoc_ps1 = foo_dir.child("misc").child("custom_pydoc.ps1");

    custom_pydoc_bat.write_str("python.exe -m pydoc %*")?;
    custom_pydoc_cmd.write_str("python.exe -m pydoc %*")?;
    custom_pydoc_ps1.write_str("python.exe -m pydoc $args")?;

    // Create the foo module
    let foo_project_src = foo_dir.child("src");
    let foo_module = foo_project_src.child("foo");
    let foo_main_py = foo_module.child("main.py");
    foo_main_py.write_str(indoc! { r#"
        import pydoc, sys

        def run():
            sys.argv[0] = "pydoc"
            pydoc.cli()

        __name__ == "__main__" and run()
       "#
    })?;

    // Install `foo` tool.
    context
        .tool_install()
        .arg(foo_dir.as_os_str())
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str())
        .env(EnvVars::PATH, bin_dir.as_os_str())
        .assert()
        .success();

    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("foo")
        .arg("does_not_exist")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str())
        .env(EnvVars::PATH, bin_dir.as_os_str()), @r###"
    success: false
    exit_code: 1
    ----- stdout -----
    The executable `does_not_exist` was not found.
    The following executables are provided by `foo`:
    - custom_pydoc.exe
    - custom_pydoc.bat
    - custom_pydoc.cmd
    - custom_pydoc.ps1
    Consider using `uv tool run --from foo <EXECUTABLE_NAME>` instead.

    ----- stderr -----
    warning: An executable named `does_not_exist` is not provided by package `foo`.
    "###);

    // Test with explicit .bat extension
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("foo")
        .arg("custom_pydoc.bat")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pydoc - the Python documentation tool

    pydoc <name> ...
        Show text documentation on something.  <name> may be the name of a
        Python keyword, topic, function, module, or package, or a dotted
        reference to a class or function within a module or module in a
        package.  If <name> contains a '\', it is used as the path to a
        Python source file to document. If name is 'keywords', 'topics',
        or 'modules', a listing of these things is displayed.

    pydoc -k <keyword>
        Search for a keyword in the synopsis lines of all available modules.

    pydoc -n <hostname>
        Start an HTTP server with the given hostname (default: localhost).

    pydoc -p <port>
        Start an HTTP server on the given port on the local machine.  Port
        number 0 can be used to get an arbitrary unused port.

    pydoc -b
        Start an HTTP server on an arbitrary unused port and open a web browser
        to interactively browse documentation.  This option can be used in
        combination with -n and/or -p.

    pydoc -w <name> ...
        Write out the HTML documentation for a module to a file in the current
        directory.  If <name> contains a '\', it is treated as a filename; if
        it names a directory, documentation is written for all the contents.


    ----- stderr -----
    "###);

    // Test with explicit .cmd extension
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("foo")
        .arg("custom_pydoc.cmd")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pydoc - the Python documentation tool

    pydoc <name> ...
        Show text documentation on something.  <name> may be the name of a
        Python keyword, topic, function, module, or package, or a dotted
        reference to a class or function within a module or module in a
        package.  If <name> contains a '\', it is used as the path to a
        Python source file to document. If name is 'keywords', 'topics',
        or 'modules', a listing of these things is displayed.

    pydoc -k <keyword>
        Search for a keyword in the synopsis lines of all available modules.

    pydoc -n <hostname>
        Start an HTTP server with the given hostname (default: localhost).

    pydoc -p <port>
        Start an HTTP server on the given port on the local machine.  Port
        number 0 can be used to get an arbitrary unused port.

    pydoc -b
        Start an HTTP server on an arbitrary unused port and open a web browser
        to interactively browse documentation.  This option can be used in
        combination with -n and/or -p.

    pydoc -w <name> ...
        Write out the HTML documentation for a module to a file in the current
        directory.  If <name> contains a '\', it is treated as a filename; if
        it names a directory, documentation is written for all the contents.


    ----- stderr -----
    "###);

    // Test with explicit .ps1 extension
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("foo")
        .arg("custom_pydoc.ps1")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pydoc - the Python documentation tool

    pydoc <name> ...
        Show text documentation on something.  <name> may be the name of a
        Python keyword, topic, function, module, or package, or a dotted
        reference to a class or function within a module or module in a
        package.  If <name> contains a '\', it is used as the path to a
        Python source file to document. If name is 'keywords', 'topics',
        or 'modules', a listing of these things is displayed.

    pydoc -k <keyword>
        Search for a keyword in the synopsis lines of all available modules.

    pydoc -n <hostname>
        Start an HTTP server with the given hostname (default: localhost).

    pydoc -p <port>
        Start an HTTP server on the given port on the local machine.  Port
        number 0 can be used to get an arbitrary unused port.

    pydoc -b
        Start an HTTP server on an arbitrary unused port and open a web browser
        to interactively browse documentation.  This option can be used in
        combination with -n and/or -p.

    pydoc -w <name> ...
        Write out the HTML documentation for a module to a file in the current
        directory.  If <name> contains a '\', it is treated as a filename; if
        it names a directory, documentation is written for all the contents.


    ----- stderr -----
    "###);

    // Test with explicit .exe extension
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("foo")
        .arg("custom_pydoc")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str())
        .env(EnvVars::PATH, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pydoc - the Python documentation tool

    pydoc <name> ...
        Show text documentation on something.  <name> may be the name of a
        Python keyword, topic, function, module, or package, or a dotted
        reference to a class or function within a module or module in a
        package.  If <name> contains a '\', it is used as the path to a
        Python source file to document. If name is 'keywords', 'topics',
        or 'modules', a listing of these things is displayed.

    pydoc -k <keyword>
        Search for a keyword in the synopsis lines of all available modules.

    pydoc -n <hostname>
        Start an HTTP server with the given hostname (default: localhost).

    pydoc -p <port>
        Start an HTTP server on the given port on the local machine.  Port
        number 0 can be used to get an arbitrary unused port.

    pydoc -b
        Start an HTTP server on an arbitrary unused port and open a web browser
        to interactively browse documentation.  This option can be used in
        combination with -n and/or -p.

    pydoc -w <name> ...
        Write out the HTML documentation for a module to a file in the current
        directory.  If <name> contains a '\', it is treated as a filename; if
        it names a directory, documentation is written for all the contents.


    ----- stderr -----
    "###);

    // Test without explicit extension (.exe should be used)
    uv_snapshot!(context.filters(), context.tool_run()
        .arg("--from")
        .arg("foo")
        .arg("custom_pydoc")
        .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
        .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str())
        .env(EnvVars::PATH, bin_dir.as_os_str()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    pydoc - the Python documentation tool

    pydoc <name> ...
        Show text documentation on something.  <name> may be the name of a
        Python keyword, topic, function, module, or package, or a dotted
        reference to a class or function within a module or module in a
        package.  If <name> contains a '\', it is used as the path to a
        Python source file to document. If name is 'keywords', 'topics',
        or 'modules', a listing of these things is displayed.

    pydoc -k <keyword>
        Search for a keyword in the synopsis lines of all available modules.

    pydoc -n <hostname>
        Start an HTTP server with the given hostname (default: localhost).

    pydoc -p <port>
        Start an HTTP server on the given port on the local machine.  Port
        number 0 can be used to get an arbitrary unused port.

    pydoc -b
        Start an HTTP server on an arbitrary unused port and open a web browser
        to interactively browse documentation.  This option can be used in
        combination with -n and/or -p.

    pydoc -w <name> ...
        Write out the HTML documentation for a module to a file in the current
        directory.  If <name> contains a '\', it is treated as a filename; if
        it names a directory, documentation is written for all the contents.


    ----- stderr -----
    "###);

    Ok(())
}
