
struct BaseLog {
  time : String,
  etype : String,
  elevel : String,
  esource : String,
  category : String,
  user  : String,
  unique : String,
  desc  : String,
  machine : String,
  guid : String,
  version: String,
  source : String,
}

struct EPLog {
  base : BaseLog,
  key  : String,
  value : String,
}
