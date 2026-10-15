# Põe a raiz no sys.path e define os perfis do Hypothesis: "dev" (padrão) e "banca" (MB1: mais casos).
import os

from hypothesis import settings

settings.register_profile("dev", max_examples=100, deadline=None, database=None)
settings.register_profile("banca", max_examples=500, deadline=None, database=None)
settings.load_profile(os.environ.get("HYPOTHESIS_PROFILE", "dev"))
