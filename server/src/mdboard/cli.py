"""mdboard CLI entrypoint."""

import argparse
import sys


def main(argv: list[str] | None = None) -> None:
    parser = argparse.ArgumentParser(
        prog="mdboard",
        description="Markdown task board for your repo",
    )
    parser.add_argument("--port", type=int, default=0, help="Port (default: auto-assign from 10600-10700)")
    parser.add_argument("--dir", default=".mdboard", help="Data directory (default: .mdboard)")
    parser.add_argument("--tasks-dir", default=None, help=argparse.SUPPRESS)

    subparsers = parser.add_subparsers(dest="command")

    serve_parser = subparsers.add_parser("serve", help="Start the board server")
    serve_parser.add_argument("--port", type=int, default=0, help="Port (default: auto-assign from 10600-10700)")
    serve_parser.add_argument("--dir", default=".mdboard", help="Data directory (default: .mdboard)")
    serve_parser.add_argument("--tasks-dir", default=None, help=argparse.SUPPRESS)

    subparsers.add_parser("init", help="Scaffold a .mdboard/ directory")

    args = parser.parse_args(argv)

    if args.command is None or args.command == "serve":
        from mdboard.server import run_server
        data_dir = args.dir
        if args.tasks_dir is not None:
            print("Warning: --tasks-dir is deprecated, use --dir instead", file=sys.stderr)
            data_dir = args.tasks_dir
        run_server(port=args.port, data_dir=data_dir)
    elif args.command == "init":
        from mdboard.init import run_init
        run_init()
