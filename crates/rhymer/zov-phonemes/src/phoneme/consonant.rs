/// The representation of a consonant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Consonant {
  /// A plosive consonant.
  Plosive(Plosive),
  /// A nasal consonant.
  Nasal(Nasal),
  /// A fricative consonant.
  Fricative(Fricative),
  /// A liquid consonant.
  Liquid(Liquid),
}

impl std::fmt::Display for Consonant {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Plosive(plosive) => write!(f, "{plosive}"),
      Self::Nasal(nasal) => write!(f, "{nasal}"),
      Self::Fricative(fricative) => write!(f, "{fricative}"),
      Self::Liquid(liquid) => write!(f, "{liquid}"),
    }
  }
}

/// The representation of a plosive consonant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Plosive {
  /// A voiceless bilabial — `[p]` i.e `papa`, `apprendre`, `lampe`.
  VoicelessBilabial,
  /// A voiced bilabial — `[b]` i.e `bien`, `arbre`, `robe`.
  VoicedBilabial,
  /// A voiceless alveolar — `[t]` i.e `table`, `autour`, `fête`, `sept`.
  VoicelessAlveolar,
  /// A voiced Alveolar — `[d]` i.e `dans`, `dire`, `garder`, `ronde`.
  VoicedAlveolar,
  /// A voicelss velar — `[k]` i.e `comme` - `que`, `lecture`, `bec`.
  VoicelessVelar,
  /// A voiced velar — `[g]` i.e `garçon`, `ogre`, `bague`.
  VoicedVelar,
}

impl std::fmt::Display for Plosive {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::VoicelessBilabial => write!(f, "p"),
      Self::VoicedBilabial => write!(f, "b"),
      Self::VoicelessAlveolar => write!(f, "t"),
      Self::VoicedAlveolar => write!(f, "d"),
      Self::VoicelessVelar => write!(f, "k"),
      Self::VoicedVelar => write!(f, "g"),
    }
  }
}

/// The representation of a nasal consonant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nasal {
  /// A bilabial — `[m]` i.e `maison`, `animal`, `pomme`.
  Bilabial,
  /// A alveolar — `[n]` i.e `nous`, `venir`, `semaine`.
  Alveolar,
  /// A palatal — `[ɲ]` i.e `gagner`, `campagne`, `panier`.
  Palatal,
}

impl std::fmt::Display for Nasal {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Bilabial => write!(f, "m"),
      Self::Alveolar => write!(f, "n"),
      Self::Palatal => write!(f, "ɲ"),
    }
  }
}

/// The representation of a fricative consonant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fricative {
  /// A voiceless labiodental — `[f]` i.e `faire`, `souffrir`, `pharmacie`.
  VoicelessLabiodental,
  /// A voiceless alveolar — `[s]` i.e `soleil`, `instant`, `fils`.
  VoicelessAlveolar,
  /// A voiceless postalveoar — `[ʃ]` i.e `chemin` - `dimanche`.
  VoicelessPostalveolar,
  /// A voiced labiodental — `[v]` i.e `vent`, `vous`, `wagon`.
  VoicedLabiodental,
  /// A voiced alveolar — `[z]` i.e `zoo`, `maison`, `des͜ œufs`.
  VoicedAlveolar,
  /// A voiced postalveolar — `[ʒ]` i.e `jardin`, `girafe`,  `rouge`.
  VoicedPostalveolar,
}

impl std::fmt::Display for Fricative {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::VoicelessLabiodental => write!(f, "f"),
      Self::VoicelessAlveolar => write!(f, "s"),
      Self::VoicelessPostalveolar => write!(f, "ʃ"),
      Self::VoicedLabiodental => write!(f, "v"),
      Self::VoicedAlveolar => write!(f, "z"),
      Self::VoicedPostalveolar => write!(f, "ʒ"),
    }
  }
}

/// The representation of a liquid consonant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Liquid {
  /// A alveolar lateral approximant — `[l]` i.e `ivre`, `vouloir`, `cheval`.
  AlveolarLateralApproximant,
  /// A alveolar trill — `[r]` i.e `rouge`, `grand`, `venir`.
  AlveolarTrill,
}

impl std::fmt::Display for Liquid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::AlveolarLateralApproximant => write!(f, "l"),
      Self::AlveolarTrill => write!(f, "r"),
    }
  }
}
