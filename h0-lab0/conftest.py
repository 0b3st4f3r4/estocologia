# Põe a raiz no sys.path e desliga o banco de exemplos do Hypothesis (pasta compartilhada).
from hypothesis import settings

settings.register_profile("h0", database=None)
settings.load_profile("h0")
