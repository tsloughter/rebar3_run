-module(rebar3_run).

-export([init/1,
         do/1,
         format_error/1]).

-export([exec/2]).

-define(PROVIDER, run).
-define(DEPS, [release]).

%% ===================================================================
%% Public API
%% ===================================================================

-spec init(rebar_state:t()) -> {ok, rebar_state:t()}.
init(State) ->
    Provider = providers:create([
                                {name, ?PROVIDER},
                                {module, ?MODULE},
                                {bare, false},
                                {deps, ?DEPS},
                                {example, "rebar3 run"},
                                {short_desc, "Run release console."},
                                {desc, ""},
                                {opts, []}
                                ]),
    State1 = rebar_state:add_provider(State, Provider),
    {ok, State1}.

-spec do(rebar_state:t()) -> {ok, rebar_state:t()} | {error, string()}.
do(State) ->
    %% Can't use on_load since rebar3 loads/unloads plugins, screwing up nifs
    init(),

    ReleaseDir = filename:join(rebar_dir:base_dir(State), "rel"),
    Config = rebar_state:get(State, relx, []),
    case lists:keyfind(release, 1, Config) of
        {release, {Name, _Vsn}, _} ->
            StartScript = filename:join([ReleaseDir, Name, "bin", Name]),
            exec(StartScript, ["console"]),
            {ok, State};
        false ->
            {error, {?MODULE, no_release}}
    end.

format_error(no_release) ->
    "No release to run was found.".

init() ->
    application:load(rebar3_run),
    PrivDir = code:priv_dir(rebar3_run),
    ok = erlang:load_nif(filename:join(PrivDir, "rebar3_run"), 0).

exec(_Path, _Args) ->
  exit(nif_library_not_loaded).
