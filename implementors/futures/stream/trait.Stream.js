(function() {var implementors = {};
implementors["futures"] = [];
implementors["quicr"] = [{text:"impl <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">FuturesStream</a> for <a class=\"struct\" href=\"quicr/struct.IncomingStreams.html\" title=\"struct quicr::IncomingStreams\">IncomingStreams</a>",synthetic:false,types:["quicr::IncomingStreams"]},{text:"impl <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">FuturesStream</a> for <a class=\"struct\" href=\"quicr/struct.IncomingSessionTickets.html\" title=\"struct quicr::IncomingSessionTickets\">IncomingSessionTickets</a>",synthetic:false,types:["quicr::IncomingSessionTickets"]},];
implementors["tokio_io"] = [{text:"impl&lt;A&gt; <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio_io/io/struct.Lines.html\" title=\"struct tokio_io::io::Lines\">Lines</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"tokio_io/trait.AsyncRead.html\" title=\"trait tokio_io::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>,&nbsp;</span>",synthetic:false,types:["tokio_io::lines::Lines"]},];
implementors["tokio_timer"] = [{text:"impl <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio_timer/struct.Interval.html\" title=\"struct tokio_timer::Interval\">Interval</a>",synthetic:false,types:["tokio_timer::interval::Interval"]},];
implementors["tokio_udp"] = [{text:"impl&lt;C:&nbsp;<a class=\"trait\" href=\"tokio_io/codec/decoder/trait.Decoder.html\" title=\"trait tokio_io::codec::decoder::Decoder\">Decoder</a>&gt; <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio_udp/struct.UdpFramed.html\" title=\"struct tokio_udp::UdpFramed\">UdpFramed</a>&lt;C&gt;",synthetic:false,types:["tokio_udp::frame::UdpFramed"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
