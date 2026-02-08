"""mdboard CLI entrypoint."""

import argparse
import sys


def main(argv: list[str] | None = None) -> None:
    parser = argparse.ArgumentParser(
        prog="mdboard",
        description="Markdown task board for your repo",
    )
    parser.add_argument("--port", type=int, default=8080, help="Port to listen on")
    parser.add_argument("--tasks-dir", default="tasks", help="Tasks root directory")

    subparsers = parser.add_subparsers(dest="command")

    serve_parser = subparsers.add_parser("serve", help="Start the board server")
    serve_parser.add_argument("--port", type=int, default=8080, help="Port to listen on")
    serve_parser.add_argument("--tasks-dir", default="tasks", help="Tasks root directory")

    subparsers.add_parser("init", help="Scaffold a tasks/ directory")

    args = parser.parse_args(argv)

    if args.command is None or args.command == "serve":
        from mdboard.server import run_server
        run_server(port=args.port, tasks_dir=args.tasks_dir)
    elif args.command == "init":
        from mdboard.init import run_init
        run_init()
