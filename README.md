rebar3_run
=====

A rebar plugin

Build
-----

    $ rebar3 compile

Use
---

Add the plugin to your rebar config:

    {plugins, [
        {rebar3_run, ".*", {git, "git://github.com/tsloughter/rebar3_run.git", {branch, "master"}}}
    ]}.

Then just call your plugin directly in an existing application:


    $ rebar3 run
    ===> Fetching rebar3_run
    ===> Compiling rebar3_run
    ===> Starting relx build process ...
    ===> Resolving OTP Applications from directories:
    .....
    Erlang/OTP 17 [erts-6.4] [source] [64-bit] [smp:4:4] [ds:4:4:10] [async-threads:10] [hipe] [kernel-poll:false]

    Eshell V6.4  (abort with ^G)
    (<RELEASE NAME>@127.0.0.1)1>
