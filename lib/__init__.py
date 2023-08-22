import json
from dataclasses import dataclass
from json_stream import streamable_list  # type: ignore


@dataclass
class Node:
    name: str


@dataclass
class Property:
    name: str


@dataclass
class State:
    temperature: float
    energy: float
    used: bool
    route: list[Node]
    # meta: dict[str, str]
    # properties: dict[Node, list[Property]]


class Chronicle(object):
    history: list[State]
    nodes: list[Node]
    edges: list[tuple[Node, Node]]
    positions: list[tuple[Node, tuple[float, float]]]

    def __init__(self, nodes: list[str], edges: list[tuple[str, str]], positions: dict[str, tuple[float, float]]):
        self.history = []
        self.nodes = [Node(s) for s in nodes]
        self.edges = [(Node(u), Node(v)) for u, v in edges]
        self.positions = [(Node(node), position)
                          for node, position in positions.items()]

    def emit(self, state: State):
        self.history.append(state)

    def save(self, file: str):
        print("Saving... (this may take awhile)")

        # fmt: off
        # json.dumps can't serialize dataclasses
        self.nodes = [node.__dict__ for node in self.nodes]  # type: ignore
        self.edges = [(a.__dict__, b.__dict__) for a, b in self.edges]  # type: ignore
        self.positions = [[node.__dict__, position] for node, position in self.positions] # type: ignore
        for i, h in enumerate(self.history):
            h.route = [node.__dict__ for node in h.route]  # type: ignore
            self.history[i] = h
        # prevent us from running out of memory
        self.history = streamable_list(state.__dict__ for state in self.history)  # type: ignore
        # fmt: on

        with open(file, "w+") as f:
            json.dump(self.__dict__, f)
