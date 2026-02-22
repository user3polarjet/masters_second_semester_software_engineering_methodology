import typing
import contextlib
import os
import pathlib
import shutil
import subprocess
import argparse
import platform
import asyncio
import pathlib
import shutil
import ctypes
import json

SCRIPT_PATH = pathlib.Path(os.path.abspath(__file__))
SCRIPT_DIR = SCRIPT_PATH.parent
BUILD_DIR = SCRIPT_DIR / 'build'

def needs_rebuild(output_path: pathlib.Path | str, input_paths: typing.Iterable[pathlib.Path | str]) -> bool:
    if not os.path.exists(output_path):
        res = True
    else:
        output_stat = os.stat(output_path)
        res = any(os.stat(input_path).st_mtime > output_stat.st_mtime for input_path in input_paths)
    if res:
        print(f'build: {output_path}')
    return res

def clean_command():
    shutil.rmtree(BUILD_DIR, ignore_errors=True)

# async def seq_cmd(*args: str | pathlib.Path, check: bool=True, **kwargs: typing.Any) -> tuple[bytes, bytes]:
#     print('cmd: ', args)
#     if check:
#         if process.returncode
#         assert process.returncode == 0
#     return stdout, stderr

async def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest='subcommand')
    parser_clean = subparsers.add_parser('clean')
    parser_clean.set_defaults(func=clean_command)
    args = parser.parse_args()
    if hasattr(args, 'func'):
        args.func()
        return
    
    BUILD_DIR.mkdir(exist_ok=True)
    target_dir = SCRIPT_DIR.parent.parent / 'typst'
    pr_list_path = BUILD_DIR / 'pr_list.json'
    if needs_rebuild(pr_list_path, ()):
    # if True:
        cmd = ("gh", "pr", "list", "--state", "all", "--limit", "5000", "--json", "number")
        print(f'{cmd=}')
        process = await asyncio.create_subprocess_exec(*cmd, cwd=target_dir, stdout=asyncio.subprocess.PIPE, stderr=asyncio.subprocess.PIPE)
        stdout, stderr = await process.communicate()
        assert process.returncode == 0
        stdout = stdout.decode()
        with open(pr_list_path, 'w') as fileio:
            fileio.write(stdout)
        pr_list = json.loads(stdout)
    else:
        with open(pr_list_path) as fileio:
            pr_list = json.load(fileio)

    import itertools
    pr_numbers = tuple(int(pr["number"]) for pr in pr_list)
    pr_paths = tuple(BUILD_DIR / f'pr_view_{number}.json' for number in pr_numbers)
    pr_rebuilds = ((number, path) for number, path in zip(pr_numbers, pr_paths) if needs_rebuild(path, ()))

    for pr_batch in itertools.batched(pr_rebuilds, 100):
        async with asyncio.TaskGroup() as tg:
            async def tg_task(number: int, pr_path: pathlib.Path):
                cmd = ("gh", "pr", "view", str(number), "--json", "number,title,author,additions,deletions,changedFiles,createdAt,mergedAt,closedAt,reviewDecision,comments,commits,reviews,labels")
                print(f'{cmd=}')
                process = await asyncio.subprocess.create_subprocess_exec(*cmd, cwd=target_dir, stdout=asyncio.subprocess.PIPE, stderr=asyncio.subprocess.PIPE,)
                stdout, stderr = await process.communicate()
                if process.returncode != 0:
                    print(f'pr {number} failed with error: {process.returncode}, {stderr.decode()=}')
                else:
                    stdout = stdout.decode()
                    with open(pr_path, 'w') as fileio:
                        fileio.write(stdout)

            for i, pr in pr_batch:
                tg.create_task(tg_task(i, pr))
        await asyncio.sleep(0.5)

    

if __name__ == '__main__':
    asyncio.run(main())

