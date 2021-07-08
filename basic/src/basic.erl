-module(basic).
-on_load(init/0).

-export([
    add/2
]).


add(_A, _B) ->
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
    LibDir = filename:join(PrivDir, "crates/basic/0.1.0/debug"),
    erlang:load_nif(filename:join(LibDir, "libbasic"), 0).


not_loaded(Line) ->
    erlang:nif_error({not_loaded, [{module, ?MODULE}, {line, Line}]}).

