use std::collections::HashMap;
use std::ops::Add;

use serde::Deserialize;
use thiserror::Error;

impl From<ServiceError> for crate::MaidenError {
    fn from(c: ServiceError) -> Self {
        crate::MaidenError::Service(c)
    }
}

#[derive(Error, Debug)]
pub enum ServiceError {}

#[derive(Deserialize, Debug, Clone)]
pub struct Service {
    http: Option<CollectionServiceHttp>,
}

impl Add for Service {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        if self.http.is_none() {
            self.http = rhs.http;
        } else if let Some(rhs_http) = rhs.http {
            self.http = self.http.map(|mut v| {
                v.extend(rhs_http.0);
                v
            });
        }
        self
    }
}

#[derive(Deserialize, Debug, Clone)]
struct CollectionServiceHttp(HashMap<String, ServiceHttp>);

impl std::iter::Extend<(std::string::String, ServiceHttp)> for CollectionServiceHttp {
    fn extend<T: IntoIterator<Item = (std::string::String, ServiceHttp)>>(&mut self, other: T) {
        other.into_iter().for_each(|(key, val)| {
            let data = if let Some(existing) = self.0.remove(&key) {
                existing.add(val)
            } else {
                val
            };
            self.0.insert(key, data);
        });
    }
}

#[derive(Deserialize, Debug, Clone)]
struct ServiceHttp {
    port: Option<u16>,
    endpoints: Option<HashMap<String, Vec<ServiceHttpEndpoint>>>,
}

impl Add for ServiceHttp {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        if rhs.port.is_some() {
            self.port = rhs.port;
        }
        if self.endpoints.is_none() {
            self.endpoints = rhs.endpoints;
        } else if let Some(rhs_endpoints) = rhs.endpoints {
            self.endpoints = self.endpoints.map(|mut endpoint| {
                endpoint.extend(rhs_endpoints);
                endpoint
            });
        }
        self
    }
}

#[derive(Deserialize, Debug, Clone)]
struct ServiceHttpEndpoint {
    method: Option<String>,
    request: Option<String>,
    response_file: Option<String>,
}

// // InputService corresponds with the data structure of unmarshalled config values.
// // It shouldn't be used directly and instead marshalled via it's parse method.
// type InputService struct {
//     Imports interface{} `yaml:"imports"`
//     Output  string      `yaml:"output"`
//     GRPC    map[string]struct {
//         Port       uint16 `yaml:"port"`
//         Definition string `yaml:"definition"`
//         Methods    map[string]struct {
//             Request  string `yaml:"request"`
//             Response string `yaml:"response"`
//         } `yaml:"methods"`
//         Conditions map[string][]struct {
//             Request  interface{} `yaml:"request"`
//             Response interface{} `yaml:"response"`
//         } `yaml:"conditions"`
//     } `yaml:"grpc"`
//     HTTP map[string]struct {
//         Port      uint16 `yaml:"port"`
//         Endpoints map[string][]struct {
//             Method       interface{} `yaml:"method"`
//             Request      interface{} `yaml:"request"`
//             Response     interface{} `yaml:"response"`
//             ResponseFile string      `yaml:"response_file"`
//         } `yaml:"endpoints"`
//     } `yaml:"http"`
// }
