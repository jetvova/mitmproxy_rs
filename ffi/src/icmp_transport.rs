use std::net::IpAddr;

use pyo3::prelude::*;
use tokio::sync::mpsc;

use mitmproxy::messages::{TransportCommand, TunnelInfo};

use crate::util::event_queue_unavailable;

#[pyclass]
#[derive(Debug)]
pub struct IcmpTransport {
    pub event_tx: mpsc::UnboundedSender<TransportCommand>,
    pub src_addr: IpAddr,
    pub dst_addr: IpAddr,
    pub tunnel_info: TunnelInfo,
}

#[pymethods]
impl IcmpTransport {
    fn send_fake_echo_reply(&self, ident: u16, seq_no: u16, data: Vec<u8>) -> PyResult<()> {
        self.event_tx
            .send(TransportCommand::SendIcmpEchoReply {
                ident,
                seq_no,
                data,

                // Directing fake reply back to the original source address
                src_addr: self.dst_addr,
                dst_addr: self.src_addr,
            })
            .map_err(event_queue_unavailable)?;
        Ok(())
    }
}
