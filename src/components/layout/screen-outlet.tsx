import { AnimatePresence, motion } from "framer-motion";
import { ActiveScreen, useNavigation } from "@/navigation";

const ScreenOutlet: React.FC = () => {
	const { route, direction } = useNavigation();

	const key =
		route.params && typeof route.params === "object"
			? `${route.name}:${Object.values(route.params).join("/")}`
			: route.name;

	return (
		<AnimatePresence mode="wait">
			<motion.div
				key={key}
				initial={{
					opacity: 0,
					y: direction === "replace" ? 0 : direction === "back" ? -10 : 10,
				}}
				animate={{ opacity: 1, y: 0 }}
				exit={{ opacity: 0, y: direction === "back" ? -10 : 10 }}
				transition={{ duration: 0.2, ease: "easeOut" }}
			>
				<ActiveScreen route={route} />
			</motion.div>
		</AnimatePresence>
	);
};

export default ScreenOutlet;
