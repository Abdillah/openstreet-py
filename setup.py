from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name = "openstreet",
    version = "0.0.1",
    rust_extensions = [ RustExtension("openstreet._binding", binding = Binding.PyO3, features = ["extension-module"]) ],
    packages = [ "openstreet" ],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe = False,
)
