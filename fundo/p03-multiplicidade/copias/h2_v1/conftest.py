# Põe a raiz no sys.path e desliga o banco de exemplos do Hypothesis (pasta compartilhada).
# Sem prazo por exemplo: o primeiro exemplo paga a compilação do numba.
import os

from hypothesis import settings

settings.register_profile("h2", database=None, deadline=None, derandomize=True)
settings.register_profile("banca", database=None, deadline=None, max_examples=500)
settings.load_profile(os.environ.get("HYPOTHESIS_PROFILE", "h2"))
