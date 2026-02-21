use crate::ports::{Reporter, ReporterFactory};

struct AdapterIndicatif {
    multi: indicatif::MultiProgress,
}

impl ReporterFactory for AdapterIndicatif {
    fn create_unchained_reporter(&self, name: &str) -> Box<dyn Reporter> {
        let pb = indicatif::ProgressBar::new_spinner();
        pb.set_style(indicatif::ProgressStyle::default_spinner()
            .template("{prefix:.bold} {spinner:.green} {msg}")
            .unwrap());
        pb.set_prefix(name.to_string());

        let added_pb = self.multi.add(pb);

        Box::new(UnchainedIndicatifReporter { progress: added_pb })
    }

    fn create_chained_reporter(&self, name: &str, len: u64) -> Box<dyn Reporter> {
        let pb = indicatif::ProgressBar::new(len);
        pb.set_style(indicatif::ProgressStyle::default_bar()
            .template("{prefix:.bold} [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .unwrap());

        pb.set_prefix(name.to_string());
        let added_pb = self.multi.add(pb);

        Box::new(IndicatifReporter { progress: added_pb })
    }
}

pub fn new() -> impl ReporterFactory {
    AdapterIndicatif{ multi: indicatif::MultiProgress::new() }
}

struct UnchainedIndicatifReporter {
    progress: indicatif::ProgressBar,
}

impl Reporter for UnchainedIndicatifReporter {
    fn stage(&self, stage_name: &str) {
        self.progress.set_message(format!("{}", stage_name));
    }

    fn info(&self, msg: &str) {
        self.progress.println(msg);
    }

    fn inc(&self, delta: u64) {
        self.progress.inc(delta);
    }

    fn finish(self: Box<Self>) {
        self.progress.finish();
    }
}

impl Drop for UnchainedIndicatifReporter {
    fn drop(&mut self) {
        if !self.progress.is_finished() {
            self.progress.finish();
        }
    }
}

struct IndicatifReporter {
    progress: indicatif::ProgressBar,
}

impl Reporter for IndicatifReporter {
    fn stage(&self, stage_name: &str) {
        self.progress.set_message(format!("{}", stage_name));
    }

    fn info(&self, msg: &str) {
        self.progress.println(msg);
    }

    fn inc(&self, delta: u64) {
        self.progress.inc(delta);
    }

    fn finish(self: Box<Self>) {
        self.progress.finish();
    }
}

impl Drop for IndicatifReporter {
    fn drop(&mut self) {
        if !self.progress.is_finished() {
            self.progress.finish();
        }
    }
}
