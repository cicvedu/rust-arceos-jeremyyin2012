use core::ops::{Deref, DerefMut};
use cfg_if::cfg_if;


pub struct NetFilter<T> {
    pub inner: T,
}

// impl<T> Deref for NetFilter<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }
//
// impl<T> DerefMut for NetFilter<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.inner
//     }
// }

cfg_if! {
    if #[cfg(feature="virtio-net")] {

        use driver_common::{BaseDriverOps, DeviceType, DevResult};
        use driver_net::{EthernetAddress, NetBufPtr, NetDriverOps};
        use driver_virtio::{MmioTransport, Transport, VirtIoNetDev};
        use crate::virtio::VirtIoHalImpl;

impl BaseDriverOps for NetFilter<VirtIoNetDev<VirtIoHalImpl, MmioTransport, 64>> {
    fn device_name(&self) -> &str {
        self.inner.device_name()
    }

    fn device_type(&self) -> DeviceType {
        self.inner.device_type()
    }
}

impl NetDriverOps for NetFilter<VirtIoNetDev<VirtIoHalImpl, MmioTransport, 64>> {
    fn mac_address(&self) -> EthernetAddress {
        self.inner.mac_address()
    }

    fn can_transmit(&self) -> bool {
        self.inner.can_transmit()
    }

    fn can_receive(&self) -> bool {
        self.inner.can_receive()
    }

    fn rx_queue_size(&self) -> usize {
        self.inner.rx_queue_size()
    }

    fn tx_queue_size(&self) -> usize {
        self.inner.tx_queue_size()
    }

    fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> DevResult {
        self.inner.recycle_rx_buffer(rx_buf)
    }

    fn recycle_tx_buffers(&mut self) -> DevResult {
        self.inner.recycle_tx_buffers()
    }

    fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult {
        warn!("Filter: transmit len[{}]", tx_buf.packet_len());
        self.inner.transmit(tx_buf)
    }

    fn receive(&mut self) -> DevResult<NetBufPtr> {
        let b = self.inner.receive()?;
        warn!("Filter: receive len[{:?}]", b.packet_len());
        Ok(b)
    }

    fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr> {
        self.inner.alloc_tx_buffer(size)
    }
}

    }
}
