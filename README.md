rebar3_run
=====

A rebar plugin

Build
-----

    $ rebar3 compile

Use
---

Add the plugin to your rebar config or `~/.config/rebar3/rebar.config`:

    {plugins, [
        rebar3_run
    ]}.

Assuming you have a `relx` config section in your `rebar.config` with `extended_start_script` set to true:

```
{relx, [{release, {<NAME>, <VERSION>}, [<YOUR APP NAME>]},
        {dev_mode, true},
        {include_erts, false},
        {extended_start_script, true}]}.
```

Then just call your plugin directly in an existing project:


    $ rebar3 run
    ===> Fetching rebar3_run
    ===> Compiling rebar3_run
    ===> Starting relx build process ...
    ===> Resolving OTP Applications from directories:
    .....
    Erlang/OTP 17 [erts-6.4] [source] [64-bit] [smp:4:4] [ds:4:4:10] [async-threads:10] [hipe] [kernel-poll:false]

    Eshell V6.4  (abort with ^G)
    (<RELEASE NAME>@127.0.0.1)1>
