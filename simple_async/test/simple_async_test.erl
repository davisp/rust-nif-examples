-module(simple_async_test).

-include_lib("eunit/include/eunit.hrl").


simple_test() ->
	{ok, R} = simple_async:new(),
	?assertEqual({more, 0}, simple_async:next(R, 0)),
	consume(R, 0).

consume(R, Val) when Val >= 100 ->
	?assertEqual({complete, Val + 1}, simple_async:next(R, Val + 1));
consume(R, Val) ->
	?assertEqual({more, Val + 1}, simple_async:next(R, Val + 1)),
	consume(R, Val + 1).
