// #[derive(Debug)]

/// # ByteSize
pub struct ByteSize {
    size: f64,
}


pub struct ByteSizeUnit {
    size: f64,
    unit: String,
}

impl ByteSize {
    /// # 解析字节单位
    pub fn parse_from_string(str: String) -> Option<ByteSize> {
        let byte_unit = ["", "K", "M", "G", "T", "P"];
        let mut index = 0;
        for (i, &byte) in str.as_bytes().iter().enumerate() {
            if !(byte >= b'0' && byte <= b'9' || byte == b'.') {
                index = i;
                break;
            }
        }
        let mut num = match str[..index].parse::<f64>() {
            Ok(num) => { num }
            Err(_) => {
                return None;
            }
        };
        let unitStr = &str[index..].trim().as_bytes();

        let mut uindex: usize = 0;
        let mut unit: usize = 0;
        for (i, x) in byte_unit.iter().enumerate() {
            if (unitStr[uindex] as char).to_string().to_ascii_uppercase() == x.to_string() {
                uindex += 1;
                unit = i;
                break;
            }
        }
        if unitStr[uindex] as char == 'i' {
            num *= 1024f64.powf(unit as f64);
            uindex += 1
        } else {
            num *= 1000f64.powf(unit as f64);
        }
        if unitStr[uindex] as char == 'B' { () } else if unitStr[uindex] as char == 'b' {
            num /= 8f64;
        } else {
            return None;
        }
        return Some(ByteSize { size: num })
    }
}


impl ByteSize {
    pub fn p_ib(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.size / 1024f64.powf(5f64),
            unit: "PiB".to_string(),
        }
    }
    pub fn t_ib(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.size / 1024f64.powf(4f64),
            unit: "TiB".to_string(),
        }
    }
    pub fn g_ib(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.size / 1024f64.powf(3f64),
            unit: "GiB".to_string(),
        }
    }
    pub fn m_ib(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.size / 1024f64.powf(2f64),
            unit: "MiB".to_string(),
        }
    }
    pub fn k_ib(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.size / 1024f64,
            unit: "KiB".to_string(),
        }
    }
    pub fn ib(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.size,
            unit: "iB".to_string(),
        }
    }
}

impl ByteSize {
    pub fn p_b(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.size / 1000f64.powf(5f64),
            unit: "PB".to_string(),
        }
    }
    pub fn t_b(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.size / 1000f64.powf(4f64),
            unit: "TB".to_string(),
        }
    }
    pub fn g_b(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.size / 1000f64.powf(3f64),
            unit: "GB".to_string(),
        }
    }
    pub fn m_b(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.size / 1000f64.powf(2f64),
            unit: "MB".to_string(),
        }
    }
    pub fn k_b(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.size / 1000f64,
            unit: "KB".to_string(),
        }
    }
    pub fn b(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.size,
            unit: "B".to_string(),
        }
    }
}


impl ByteSizeUnit {
    /// # 拼装成字符串
    pub fn to_string(&self) -> String {
        format!("{}{}", self.size, self.unit)
    }
}