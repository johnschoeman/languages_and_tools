from setuptools import setup
from setuptools import find_packages

setup(
    name='mypackage',
    version='0.0.1',
    install_requires=[
        'requests',
        'importlib-metadata; python_version == "3.8"',
    ],
    entry_points={
        'console_scripts': [
            'myhello = hello.world:main',
            'myworld = hello:main'
        ]
    }
)
