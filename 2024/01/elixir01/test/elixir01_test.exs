defmodule Elixir01Test do
  use ExUnit.Case
  doctest Elixir01

  test "greets the world" do
    assert Elixir01.hello() == :world
  end
end
