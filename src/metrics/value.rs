use std::io;
use std::io::Error;
use std::time;

pub trait MetricValue {
    fn write<T>(self, w: T) -> io::Result<()>
    where
        T: io::Write;
}

impl MetricValue for u64 {
    fn write<T>(self, w: T) -> io::Result<()>
    where
        T: io::Write,
    {
        itoa::write(w, self).map(|_| ())
    }
}
impl MetricValue for i32 {
    fn write<T>(self, w: T) -> io::Result<()>
    where
        T: io::Write,
    {
        itoa::write(w, self).map(|_| ())
    }
}

impl MetricValue for usize {
    fn write<T>(self, w: T) -> io::Result<()>
    where
        T: io::Write,
    {
        itoa::write(w, self).map(|_| ())
    }
}

impl MetricValue for f64 {
    fn write<T>(self, w: T) -> io::Result<()>
    where
        T: io::Write,
    {
        dtoa::write(w, self).map(|_| ())
    }
}

impl MetricValue for time::Duration {
    fn write<T>(self, w: T) -> io::Result<()>
    where
        T: io::Write,
    {
        dtoa::write(w, self.as_secs_f64()).map(|_| ())
    }
}

impl MetricValue for &str {
    fn write<T>(self, mut w: T) -> io::Result<()>
    where
        T: io::Write,
    {
        w.write_all(self.as_bytes())
    }
}

impl<V> MetricValue for &V
where
    V: MetricValue + Copy,
{
    fn write<T>(self, w: T) -> Result<(), Error>
    where
        T: io::Write,
    {
        (*self).write(w)
    }
}
