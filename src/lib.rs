//! EE895 CO2, Pressure and Temperature driver.
//!
//! This crate provides a driver for the EE895 sensor. The CO2, temperature and pressure measurements can be from the sensor over I2C.
//! The main driver is created using [`Ee895::new`] which accepts an I2C interface instance.
//! 
//! 
#![deny(unsafe_code)]
#![no_std]
#![no_main]

use embedded_hal::blocking::i2c;

/// EE895 device driver implemented the I2C Simplified Protocol
/// Datasheet: https://www.epluse.com/fileadmin/data/product/ee895/BA_EE895.pdf

const ADDRESS: u8 = 0x5E;

#[allow(dead_code, non_camel_case_types)]
#[derive(Copy, Clone)]
/// Register addresses
pub enum Register {
    CO2_MSB = 0x00,
    CO2_LSB = 0x01,
    TEMP_MSB = 0x02,
    TEMP_LSB = 0x03,
    PRESSURE_MSB = 0x06,
    PRESSURE_LSB = 0x07,
}

impl Register {
    /// Get register address.
    fn addr(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug)]
pub struct EE895<I2C> {
    // The concrete I²C device implementation.
    pub i2c: I2C,
}

impl<I2C, E> EE895<I2C>
where
    I2C: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    /// Creates a new driver from a I2C peripheral
    pub fn new(i2c: I2C) -> Result<Self, E> {
        let ee895 = Self { i2c };
        Ok(ee895)
    }

    fn read_value(&mut self, register_msb: Register, register_lsb: Register) -> Result<f32, E> {
        let mut buffer_msb = [0];
        let mut buffer_lsb = [0];
        self.i2c
            .write_read(ADDRESS, &[register_msb.addr()], &mut buffer_msb)?;
        self.i2c
            .write_read(ADDRESS, &[register_lsb.addr()], &mut buffer_lsb)?;
        let value = ((buffer_msb[0] as i16) << 8) | buffer_lsb[0] as i16;
        Ok(value as f32)
    }

    /// Reads the Temperature from the Sensor in Degree Celsius
    pub fn read_temperature(&mut self) -> Result<f32, E> {
        let value = self.read_value(Register::TEMP_MSB, Register::TEMP_LSB)?;
        Ok(value / 100.0)
    }

    /// Reads the CO2 from the Sensor in ppM
    pub fn read_co2(&mut self) -> Result<f32, E> {
        let value = self.read_value(Register::CO2_MSB, Register::CO2_LSB)?;
        Ok(value)
    }

    /// Reads the Pressure from the Sensor in hPa
    pub fn read_pressure(&mut self) -> Result<f32, E> {
        let value = self.read_value(Register::PRESSURE_MSB, Register::PRESSURE_LSB)?;
        Ok(value / 10.0)
    }
}
