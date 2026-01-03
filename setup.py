from setuptools import setup, find_packages

setup(
    name="crypto-tree",
    version="0.1.0",
    description="A cryptographically secure, self-balancing AVL tree for blockchain transaction indexing. O(log n) search with Merkle proofs.",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    author="Your Name",
    author_email="your.email@example.com",
    url="https://github.com/yourusername/crypto-tree",
    packages=find_packages(where="src"),
    package_dir={"": "src"},
    python_requires=">=3.8",
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Topic :: Security :: Cryptography",
        "Topic :: Database",
        "Topic :: Software Development :: Libraries",
    ],
    install_requires=[],
    extras_require={
        "dev": [
            "pytest",
            "black",
            "mypy",
            "ruff",
        ],
    },
)
