Getting Started
===============


Installation
------------
We have yet to provide pypi project due to this one still in alpha stage.
But we provide ``.whl`` download straight from this repo so you can put
it in the requirements.txt or do ``pip install urlto.whl``.

See the `release page <https://github.com/Abdillah/openstreet-py/releases>`_ for ``whl`` file available.

.. code-block:: bash

    $ pip install https://github.com/Abdillah/openstreet-py/releases/download/v0.0.1-nightly.20210517/openstreet-0.1-cp38-cp38-linux_x86_64.whl

Usage
-----
Our library consist of two parts: query system and graph system.
For both system, you can study more on reference

Query
^^^^^
Map contains three main information: nodes, ways, and bounds.
Nodes and ways are usually very large numbers in a map.
Therefor, a robust and performant system is required to get
certain nodes and ways. This library provide "lazy" query system
using fluent interface pattern that makes the query only
get executed when all filter has been setup in place.

.. code-block:: python

    map = Map("/path/to/map.osm")
    streets = map.ways().where_tag_in("highstreet", [ "primary", "secondary" ]).get()

Tag is an element in OSM format looked like these:
``<tag key="akeyhere" value="somevalue" />``. Tag is always the child
of ``<node />`` or ``<way />`` element. So, one way to filter the node and
way is by finding the value of the tag. So using the ``by_tag_in`` filter
would means looping over all the ways in the OSM with the matching tag "highstreet"
and value of "primary" or "secondary".

Another way to filter the node and way is by finding using their id.
This is provided by :py:meth:`openstreet.NodeQueryBuilder.by_id` and its way's counterpart.

**Query References**

* :py:class:`openstreet.NodeQueryBuilder`
* :py:class:`openstreet.WayQueryBuilder`
* `Examples <https://github.com/Abdillah/openstreet-py/blob/d81b4a7877cedfba2310e45666dff6ff1149d8d8/examples/>`_

Graph
^^^^^
The street network can be represented in a graph, and then applied several basic operation.
In which the most popular of it, is shortest path. To use shortest path action, you can directly
call from the Map object.

.. code-block:: python

    map = Map("/path/to/map.osm")
    ab_shortest_path = map.shortest_path(node_id_a, node_id_b)

The resulting variable will contains node id that can be travelled.
Please note however, that currently, we only select node that is an intersection of the street.
This makes our algorithm run faster as well as has smaller memory.

For identifying which way to travel given two nodes from ``ab_shortest_path``,
you can use :py:meth:`openstreet.WayQueryBuilder.contain_nodes` function.

**Graph System Reference**

* :py:class:`openstreet.Graph`
* `Examples <https://github.com/Abdillah/openstreet-py/blob/d81b4a7877cedfba2310e45666dff6ff1149d8d8/examples/>`_
