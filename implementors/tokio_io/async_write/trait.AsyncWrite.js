(function() {var implementors = {};
implementors["quicr"] = ["impl <a class=\"trait\" href=\"tokio_io/async_write/trait.AsyncWrite.html\" title=\"trait tokio_io::async_write::AsyncWrite\">AsyncWrite</a> for <a class=\"struct\" href=\"quicr/struct.Stream.html\" title=\"struct quicr::Stream\">Stream</a>","impl <a class=\"trait\" href=\"tokio_io/async_write/trait.AsyncWrite.html\" title=\"trait tokio_io::async_write::AsyncWrite\">AsyncWrite</a> for <a class=\"struct\" href=\"quicr/struct.SendStream.html\" title=\"struct quicr::SendStream\">SendStream</a>",];
implementors["tokio_reactor"] = ["impl&lt;E&gt; <a class=\"trait\" href=\"tokio_io/async_write/trait.AsyncWrite.html\" title=\"trait tokio_io::async_write::AsyncWrite\">AsyncWrite</a> for <a class=\"struct\" href=\"tokio_reactor/struct.PollEvented.html\" title=\"struct tokio_reactor::PollEvented\">PollEvented</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"mio/event_imp/trait.Evented.html\" title=\"trait mio::event_imp::Evented\">Evented</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,&nbsp;</span>","impl&lt;'a, E&gt; <a class=\"trait\" href=\"tokio_io/async_write/trait.AsyncWrite.html\" title=\"trait tokio_io::async_write::AsyncWrite\">AsyncWrite</a> for &amp;'a <a class=\"struct\" href=\"tokio_reactor/struct.PollEvented.html\" title=\"struct tokio_reactor::PollEvented\">PollEvented</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"mio/event_imp/trait.Evented.html\" title=\"trait mio::event_imp::Evented\">Evented</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a </a>E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,&nbsp;</span>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
