/// The representation of a vowel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vowel {
  /// An oral vowel.
  Oral(Oral),
  /// A nasal vowel.
  Nasal(Nasal),
}

impl std::fmt::Display for Vowel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Oral(oral) => write!(f, "{oral}"),
      Self::Nasal(nasal) => write!(f, "{nasal}"),
    }
  }
}

/// The representation of a oral vowel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Oral {
  /// A close front unrounded — `[i]` i.e `il`, `ici`, `le nid`, `lui`.
  CloseFrontUnrounded,
  /// A close mid front unrounded — `[e]` i.e `école`, `année`, `donner`.
  CloseMidFrontUnrounded,
  /// An open mid front unrounded — `[ɛ]` i.e `air`, `elle`, `mère`, `peine`.
  OpenMidFrontUnrounded,
  /// An open front unrounded — `[a]` i.e `aller`, `patte`, `moi`.
  OpenFrontUnrounded,
  /// An open back unrounded — `[ɑ]` i.e `âne`, `(un) gars`, `pâté`.
  OpenBackUnrounded,
  /// A close mid back rounded — `[o]` i.e `odeur`, `jaune`, `beau`.
  CloseMidBackRounded,
  /// An open mid back rounded — `[ɔ]` i.e `ordre`, `comme`, `le sol`.
  OpenMidBackRounded,
  /// A close back rounded — `[u]` i.e `ou`, `ouvrir`, `la poule`, `le coût`.
  CloseBackRounded,
  /// A close front rounded — `[y]` i.e `une`, `du`, `nature`, `rue`.
  CloseFrontRounded,
  /// A close mid front rounded — `[ø]` i.e `(des) œufs`, `du feu`.
  CloseMidFrontRounded,
  /// An open mid front rounded — `[œ]` i.e `leur`, `cœur`, `le fleuve`.
  OpenMidFrontRounded,
}

impl std::fmt::Display for Oral {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::CloseFrontUnrounded => write!(f, "i"),
      Self::CloseMidFrontUnrounded => write!(f, "e"),
      Self::OpenMidFrontUnrounded => write!(f, "ɛ"),
      Self::OpenFrontUnrounded => write!(f, "a"),
      Self::OpenBackUnrounded => write!(f, "ɑ"),
      Self::CloseMidBackRounded => write!(f, "o"),
      Self::OpenMidBackRounded => write!(f, "ɔ"),
      Self::CloseBackRounded => write!(f, "u"),
      Self::CloseFrontRounded => write!(f, "y"),
      Self::CloseMidFrontRounded => write!(f, "ø"),
      Self::OpenMidFrontRounded => write!(f, "œ"),
    }
  }
}

/// The representation of a nasal vowel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nasal {
  /// An open back — `[ɑ̃]` i.e `en`, `enfant`, `grand`, `encore`.
  OpenBack,
  /// An open mid front — `[ɛ̃]` i.e `instant`, `peinture`,  `pain`, `chien`.
  OpenMidFront,
  /// An open mid front rounded — `[œ̃]` i.e .
  OpenMidFrontRounded,
  /// An open mid back — `[ɔ̃]` i.e `on`, `oncle`, `répondre`, `pompe`.
  OpenMidBack,
}

impl std::fmt::Display for Nasal {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::OpenBack => write!(f, "ɑ̃"),
      Self::OpenMidFront => write!(f, "ɛ̃"),
      Self::OpenMidFrontRounded => write!(f, "œ̃"),
      Self::OpenMidBack => write!(f, "ɔ̃"),
    }
  }
}
