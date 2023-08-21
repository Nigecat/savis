import json
from dataclasses import dataclass
from json_stream import streamable_list # type: ignore

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
    nodes: list[Node]    
    history: list[State]
    edges: list[tuple[Node, Node]]

    def __init__(self, nodes: list[Node], edges: list[tuple[Node, Node]]):
        self.nodes = nodes
        self.edges = edges
        self.history = []

    def emit(self, state: State):
        self.history.append(state)

    def save(self, file: str):
        # json.dumps can't serialize dataclasses
        self.nodes = [node.__dict__ for node in self.nodes] # type: ignore
        self.edges = [(a.__dict__, b.__dict__) for a, b in self.edges] # type: ignore
        for i, h in enumerate(self.history):
            h.route = [node.__dict__ for node in h.route] # type: ignore
            self.history[i] = h
        # prevent us from running out of memory
        self.history = streamable_list(state.__dict__ for state in self.history) # type: ignore

        with open(file, "w+") as f:
            json.dump(self.__dict__, f)
