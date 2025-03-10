from phonemizer import phonemize

def phonemize_french(text):
  phonemes = phonemize(
    text,
    language="fr",
    backend="espeak",
    separator=Separator(phone=None, word=' ', syllable='|'),
    strip=True,
    preserve_punctuation=True,
    njobs=4
  )

  return phonemes.split()  # Return phonemes as a list of words

print(phonemize_french("Bonjour pour toujours"))
