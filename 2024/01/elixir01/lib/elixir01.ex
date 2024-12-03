defmodule Elixir01 do
  @moduledoc """
  Solutions for Advent of Code 2024, Day 1 in Elixir..

  """

  def main do
    run()
  end

  def run do
    input_path = "./input/input.txt"

    lines =
      input_path
      |> File.stream!()
      |> Stream.map(&String.trim/1)
      |> Enum.map(fn line ->
        [left, right] = String.split(line) |> Enum.map(&String.to_integer/1)
        {left, right}
      end)

    left = lines |> Enum.map(&elem(&1, 0)) |> Enum.sort()
    right = lines |> Enum.map(&elem(&1, 1)) |> Enum.sort()

    # Step one: Total distance
    step_one =
      left
      |> Enum.zip(right)
      |> Enum.map(fn {l, r} -> abs(l - r) end)
      |> Enum.sum()

    IO.puts("Step one = #{step_one}")

    # Step two: Similarity score
    step_two =
      left
      |> Enum.map(fn x -> x * Enum.count(right, fn y -> y == x end) end)
      |> Enum.sum()

    IO.puts("Step two = #{step_two}")
  end
end

