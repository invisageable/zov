from phonemizer import phonemize
from phonemizer.separator import Separator

def phonemize_french(text):
  print(f"from python:\n {text}")

  phonemes = phonemize(
    text["lines"],
    language="fr-fr",
    backend="espeak",
    # note(ivs) — it seems that syllables split do not work with "espeak" backend.
    separator=Separator(phone=None, word="", syllable="|"),
    # strip=False,
    # preserve_empty_lines=True,
    # preserve_punctuation=True,
    # prepend_text=True,
  )

  return phonemes
