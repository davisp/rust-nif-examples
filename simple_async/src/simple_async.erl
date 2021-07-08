-module(simple_async).
-on_load(init/0).

-export([
    new/0,
    next/2
]).


new() ->
    not_loaded(?LINE).

next(_Nif, _Val) ->
    not_loaded(?LINE).

init() ->
    PrivDir = case code:priv_dir(?MODULE) of
        {error, _} ->
            EbinDir = filename:dirname(code:which(?MODULE)),
            AppPath = filename:dirname(EbinDir),
            filename:join(AppPath, "priv");
        Path ->
            Path
    end,
    LibDir = filename:join(PrivDir, "crates/simple_async"),
    erlang:load_nif(filename:join(LibDir, "simple_async"), 0).


not_loaded(Line) ->
    erlang:nif_error({not_loaded, [{module, ?MODULE}, {line, Line}]}).

